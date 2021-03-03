use std::{
    fmt::{self, Display},
    rc::Rc,
    str::{pattern::Pattern, FromStr},
};
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
        }
    }

    pub fn parse(&self) -> Program<'_> {
        let mut expressions = Vec::new();
        let mut expr_stack = Vec::new();

        for token in self.tokenizer.tokenize() {
            use Node::*;
            match token {
                Token::LParen => expr_stack.push(Expression::new()),
                // Pop the current working expression back into the previous
                // one, or move it to the completed expressions vector if there
                // is no previous expression in the stack (which is to say: the
                // current expression is top-level).
                Token::RParen => match expr_stack.pop() {
                    Some(expression) => {
                        if expr_stack.is_empty() {
                            expressions.push(expression)
                        } else {
                            expr_stack.push_inside(Node::Expr(expression))
                        }
                    }
                    None => panic!(
                        "got RParen token without matching LParen, \
                    here's the expr_stack: {:#?}, and expressions: {:#?}",
                        expr_stack, expressions
                    ),
                },
                Token::Word(val) => expr_stack.push_inside(Ident(val)),
                Token::Int64(val) => expr_stack.push_inside(Int64(val)),
                token if token.is_keyword() => expr_stack.push_inside(Builtin(token)),
                _ => unimplemented!(),
            }
        }

        Program::new(expressions)
    }
}

trait PushInside<'a> {
    fn push_inside(&mut self, node: Node<'a>);
}

impl<'a> PushInside<'a> for Vec<Expression<'a>> {
    /// Pushes inside of the expression at the top of the stack.
    fn push_inside(&mut self, node: Node<'a>) {
        self.last_mut().unwrap().push(node);
    }
}

/// A representation of our Lispy program, which consists of top-level
/// expressions.
#[derive(Debug, PartialEq, Eq)]
pub struct Program<'a>(Vec<Expression<'a>>);
impl<'a> Program<'a> {
    fn new(expressions: Vec<Expression<'a>>) -> Self {
        Self(expressions)
    }

    pub fn expressions(&self) -> &[Expression<'a>] {
        &self.0
    }

    /// Utility for turning a single expression node into a program. Useful for
    /// testing.
    fn from_node(node: Node<'a>) -> Self {
        Self::new(if let Node::Expr(expression) = node {
            vec![expression]
        } else {
            panic!("expected an Expr Node")
        })
    }

    /// Expects all top-level Nodes to be Exprs.
    fn from_nodes(nodes: Vec<Node<'a>>) -> Self {
        Self::new(
            nodes
                .into_iter()
                .map(|node| {
                    if let Node::Expr(expression) = node {
                        expression
                    } else {
                        panic!("expected all top-level Nodes to be Expr Nodes")
                    }
                })
                .collect(),
        )
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for expr in self.expressions() {
            writeln!(f, "{}", Node::Expr(expr.clone()))?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node<'a> {
    Ident(&'a str),
    Int64(i64),
    Builtin(Token<'a>),
    Expr(Expression<'a>),
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Node::*;
        match self {
            Ident(val) => write!(f, "{}", val),
            Int64(val) => write!(f, "{}", val),
            Builtin(token) => write!(f, "{}", token),
            Expr(expression) => {
                let nodes = expression.nodes();
                let len = nodes.len();

                write!(f, "(")?;
                for (i, node) in nodes.iter().enumerate() {
                    let last = i + 1 == len;

                    if last {
                        write!(f, "{})", node)?;
                    } else {
                        write!(f, "{} ", node)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl<'a> Node<'a> {
    /// Builds an Expr Node.
    fn expr(vec: Vec<Node<'a>>) -> Node<'a> {
        Node::Expr(Expression(vec))
    }

    pub fn get_i64(&self) -> i64 {
        match self {
            Node::Int64(val) => *val,
            _ => unimplemented!(),
        }
    }

    pub fn get_expr(&self) -> Expression<'a> {
        match self {
            Node::Expr(expression) => expression.clone(),
            _ => unimplemented!(),
        }
    }

    pub fn get_ident(&self) -> &'a str {
        match self {
            Node::Ident(ident) => *ident,
            _ => unimplemented!(),
        }
    }
}

/// An expression is just parenthesis surrounding AST nodes: (thing 33 "yes").
/// Expressions are themselves AST nodes, so they may be nested (inside of the
/// Node::Expr).
///
/// We keep this as a separate type so that we may clearly delineate it from
/// other Nodes, as our lispy programs do not consider any other AST node to be
/// valid at the top-level.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression<'a>(Vec<Node<'a>>);

impl<'a> Expression<'a> {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn nodes(&self) -> &[Node<'a>] {
        &self.0
    }

    fn push(&mut self, node: Node<'a>) {
        self.0.push(node);
    }
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Expression::new()
    }
}

/// The reason I use a struct instead of just a function for tokenization is
/// because we want to ensure that our input data is ran through
/// `ensure_each_token_has_whitespace_surrounding` before tokenizing, but
/// because that function returns a String, *something* has to be responsible
/// for that strings ownership if we use any references to it.
///
/// So, our options are either create a newly allocated String for *each*
/// identifier, or hold ownership of our String so that we can maintain
/// references to it. We opt for the latter for performance reasons.
///
/// This is a key example of how Rust's borrowing semantics leads to design
/// patterns could be completely avoided in other languages. That is to say: in
/// any other language, our tokenization process could just be a simple
/// function. However, in Rust, this is impossible without unsafe code when
/// using simple functions like `str::replace`, which by allocating strings,
/// force us to manage ownership of those Strings. Returning references to them
/// would immediately yield dangling pointers otherwise.
///
/// In exchange for control, Rust hits our ergonomics here.
struct Tokenizer {
    source: String,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Self {
            source: ensure_each_token_has_whitespace_surrounding(input),
        }
    }

    fn tokenize(&self) -> Vec<Token<'_>> {
        let mut tokens = Vec::new();

        let mut skipping_comment = false;
        for word in self.source.split_whitespace() {
            // Flag comments for skipping.
            if word == "//" {
                skipping_comment = true;
            } else if word == r"\\" {
                skipping_comment = false;
                continue; // Skip this non-token.
            }

            // Skip over commas and comments
            if word == "," || skipping_comment {
                continue;
            }

            // Safe because split_whitespace will not produce empty strings.
            let first_char = word.chars().nth(0).unwrap();
            let word_lower = word.to_lowercase();

            if word == "(" {
                tokens.push(Token::LParen);
            } else if word == ")" {
                tokens.push(Token::RParen);
            } else if word_lower == "print" {
                tokens.push(Token::Print);
            } else if word_lower == "cons" {
                tokens.push(Token::Cons)
            } else if word_lower == "add" || word == "+" {
                tokens.push(Token::Add)
            } else if word_lower == "def" {
                tokens.push(Token::Def)
            } else if word_lower == "defun" {
                tokens.push(Token::Defun)
            }
            // Is a positive number.
            else if word.chars().all(|ch| ch.is_ascii_digit()) {
                tokens.push(Token::Int64(word.parse().unwrap()));
            }
            // Is a positive or negative number (explicit sign).
            else if (first_char == '-' || first_char == '+')
                && word.len() > 1 // Make sure we don't solely match a + or -.
                && word.chars().skip(1).all(|ch| ch.is_ascii_digit())
            {
                tokens.push(Token::Int64(word.parse().unwrap()));
            } else if is_valid_identifier(first_char, word) {
                tokens.push(Token::Word(word));
            } else {
                panic!("failed to tokenize '{}'", word);
            }
        }
        tokens
    }
}

const OPERATOR_CHARS: &'static str = "'+-/*!@#$%&<>?=:;|_";

fn is_valid_identifier(first_char: char, word: &str) -> bool {
    (first_char.is_alphanumeric() || first_char.is_contained_in(OPERATOR_CHARS))
        && word
            .chars()
            .skip(1)
            .all(|ch| ch.is_alphanumeric() || ch.is_contained_in(OPERATOR_CHARS))
}

/// In our lisp, the only tokens that are allowed to not have whitespace between
/// them and other tokens are LParen and RParen (at least initially). This makes
/// parsing more challenging, so we insert whitespace so that we can easily
/// split out *all* possible tokens.
fn ensure_each_token_has_whitespace_surrounding(source: &str) -> String {
    source
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace(',', " , ")
        .replace("//", " // ")
        .replace(r"\\", r" \\ ")
}

use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, PartialEq, Eq, Clone, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Token<'a> {
    Word(&'a str),
    Int64(i64),
    #[strum(serialize = "(")]
    LParen,
    #[strum(serialize = ")")]
    RParen,

    // Keywords:
    Add,
    Print,
    Cons,
    Def,
    Defun,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Word(val) => write!(f, "{}", val),
            Int64(val) => write!(f, "{}", val),
            rest => write!(f, "{}", rest.as_ref()),
        }
    }
}

impl<'a> Token<'a> {
    const KEYWORDS: &'a [Token<'a>] = &[
        Token::Add,
        Token::Print,
        Token::Cons,
        Token::Def,
        Token::Defun,
    ];
    fn is_keyword(&self) -> bool {
        Self::KEYWORDS.contains(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        use Token::*;
        let input = "+984 -156()))((adam app11e";
        let tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        let expected = vec![
            Int64(984),
            Int64(-156),
            LParen,
            RParen,
            RParen,
            RParen,
            LParen,
            LParen,
            Word("adam"),
            Word("app11e"),
        ];
        for (i, token) in expected.into_iter().enumerate() {
            assert_eq!(token, tokens[i]);
        }
    }

    #[test]
    fn test_parsing() {
        use Node::*;
        let source = "(+ 3 (why 9 ten) yes)";
        let parser = Parser::new(source);
        let expected = Node::expr(vec![
            Ident("+"),
            Int64(3),
            Node::expr(vec![Ident("why"), Int64(9), Ident("ten")]),
            Ident("yes"),
        ]);

        assert_eq!(parser.parse(), Program::from_node(expected));
    }
}
