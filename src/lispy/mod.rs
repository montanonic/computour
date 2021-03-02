mod calculator;
mod understanding_lifetimes;

use std::{mem, str::pattern::Pattern};

pub fn main() {
    let source = "(+ 3 (why 9 ten) yes)";
    let parser = Parser::new(source);
    println!("{:?}", parser.parse());
}

struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
        }
    }

    fn parse(&self) -> Vec<Node<'_>> {
        let mut expressions = Vec::new();
        let mut expr_stack = Vec::new();
        let mut curr_expr = None;

        for token in self.tokenizer.tokenize() {
            use Node::*;
            match token {
                Token::LParen => {
                    if curr_expr.is_some() {
                        expr_stack.push(curr_expr.replace(Expression::new()).unwrap());
                    } else {
                        curr_expr = Some(Expression::new());
                    }
                }
                Token::RParen => match expr_stack.pop() {
                    Some(expr) => expressions.push(Node::Expr(expr)),
                    None => panic!(
                        "got RParen token without matching LParen, \
                    here's the curr_expression: {:#?}, expr_stack: {:#?}, and expressions: {:#?}",
                        curr_expr, expr_stack, expressions
                    ),
                },
                Token::Word(val) => curr_expr.as_mut().unwrap().push(Ident(val)),
                Token::Int64(val) => curr_expr.as_mut().unwrap().push(Int64(val)),
            }
        }

        expressions
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

#[derive(Debug, PartialEq)]
enum Node<'a> {
    Ident(&'a str),
    Int64(i64),
    Expr(Expression<'a>),
}

impl<'a> Node<'a> {
    /// Builds an Expr Node.
    fn expr(vec: Vec<Node<'a>>) -> Node<'a> {
        Node::Expr(Expression(vec))
    }
}

/// An expression is just parenthesis surrounding AST nodes: (thing 33 "yes").
/// Expressions are themselves AST nodes, so they may be nested.
#[derive(Debug, PartialEq)]
struct Expression<'a>(Vec<Node<'a>>);

impl<'a> Expression<'a> {
    fn new() -> Self {
        Self(Vec::new())
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

/// Extracts an expression from the current point, expecting the token slice to
/// begin with an LParen.
fn extract_expression<'a>(tokens: &'a [Token<'a>]) -> Option<&'a [Token<'a>]> {
    assert_eq!(tokens.get(0), Some(&Token::LParen));

    let mut parens_bal = 1;
    let mut end_index = 1;
    for token in tokens.iter().skip(1) {
        if parens_bal == 0 {
            break;
        }
        match token {
            Token::LParen => parens_bal += 1,
            Token::RParen => parens_bal -= 1,
            _ => {}
        }
        end_index += 1;
    }

    if parens_bal == 0 {
        Some(&tokens[0..end_index])
    } else {
        None
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

        for word in self.source.split_whitespace() {
            // Safe because split_whitespace will not produce empty strings.
            let first_char = word.chars().nth(0).unwrap();

            dbg!(word);

            if word == "(" {
                tokens.push(Token::LParen);
            } else if word == ")" {
                tokens.push(Token::RParen);
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
                unimplemented!()
            }
        }
        tokens
    }
}

const OPERATOR_CHARS: &'static str = "+-/*!@#$%&";

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
    let source = source.replace('(', " ( ");
    let source = source.replace(')', " ) ");
    source
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Word(&'a str),
    Int64(i64),
    LParen,
    RParen,
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
    fn test_extract_expression() {
        use Token::*;
        let tokens = vec![LParen, RParen];
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens);

        let tokens = vec![LParen, LParen, RParen];
        assert!(extract_expression(&tokens).is_none());
        let tokens = vec![LParen, LParen, RParen, Int64(1234), RParen];
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens);
        let tokens = vec![LParen, RParen, LParen];
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens[0..2]);

        let tokenizer = Tokenizer::new("(+ (- 3 4) 5 6)");
        let tokens = tokenizer.tokenize();
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens);
    }

    #[test]
    fn test_parsing() {
        use Node::*;
        let source = "(+ 3 (why 9 ten) yes)";
        let parser = Parser::new(source);
        let expected = vec![
            Ident("+"),
            Node::expr(vec![Ident("why"), Int64(9), Ident("ten")]),
            Ident("yes"),
        ];

        assert_eq!(parser.parse(), expected);
    }
}
