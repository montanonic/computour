//! Curious to try an imperative, unstructured implementation and without lexing.

use std::mem;
use tokens::TokenStream;

pub fn run() {
    let code = r"
Yes!
";
    interpret(code);
}

fn interpret(code: &str) {}

mod tokens {
    use self::Token::*;

    use std::str::Lines;

    type Tokens<'code> = Vec<Token<'code>>;

    #[derive(Debug, Clone, Copy)]
    pub enum Token<'code> {
        LParen,
        RParen,
        /// An alphanumerical word.
        Word(&'code str),
        /// Must only consist of digits.
        Number(&'code str),
        /// End of line.
        EOL,
    }

    /// Streams off the tokens lazily. Stores the tokens internally, allowing
    /// for extension to arbitrary backtracking if desired. Because of this
    /// storage, this stream is not zero-copy.
    pub struct TokenStream<'code> {
        /// Have the lifetime of the input string.
        lines: Lines<'code>,
        /// Vec for holding onto our token results.
        tokens: Tokens<'code>,
        /// Position within our tokens buffer.
        pos: usize,
    }

    impl<'code> TokenStream<'code> {
        pub fn new(string: &'code str) -> Tokens<'code> {
            let mut this = Self {
                lines: string.lines(),
                tokens: Vec::new(),
                pos: 0,
            };
            todo!()
        }

        /// Gets the next token.
        pub fn next(&mut self) -> Option<Token<'code>> {
            // We already have spare tokens, just read them off.
            if self.pos < self.tokens.len() {
                let pos = self.pos;
                self.pos += 1;
                Some(self.tokens[pos])
            }
            // We need more tokens.
            else if self.pos == self.tokens.len() {
                let prev_token_count = self.tokens.len();
                match self.consume_line_into_tokens() {
                    // There was another line of input, keep reading.
                    Some(()) => {
                        // Safeguard just to make sure we never indefinitely recurse with self.next().
                        assert!(
                            self.tokens.len() > prev_token_count,
                            "consume_line_into_tokens should have pushed more tokens to our vector"
                        );
                        self.next()
                    }
                    // No more lines, we're done lexing.
                    None => None,
                }
            } else {
                panic!("self.pos should never exceed the length of self.tokens")
            }
        }

        /// Reads the next line, pushing every lexed token into `self.tokens`.
        fn consume_line_into_tokens(&mut self) -> Option<()> {
            self.lines.next().map(|line| {
                // We don't want to include an EOL token for anything that is solely a newline.
                if line.is_empty() {
                    return;
                }
                for word in line.split_whitespace() {
                    self.tokenize_word(word);
                }
                self.tokens.push(EOL);
            })
        }

        /// Appends directly to our buffer to avoid extra allocations when words
        /// split into multliple tokens.
        fn tokenize_word(&mut self, word: &'code str) {
            // Words with any parentheses.
            if let Some(ptype) = has_paren(word) {
                self.tokens.extend(Self::handle_paren(word, ptype))
            }
            // Words of only digits.
            else if is_number(word) {
                self.tokens.push(Number(word))
            }
            // Arbitrary alphanumeric words that don't satisfy the above cases.
            else {
                self.tokens.push(Word(word))
            }
        }

        /// Based on the type of paren contained in the stream, expects all other paren
        fn handle_paren(word: &'code str, ptype: ParenType) -> impl Iterator<Item = Token<'code>> {
            use ParenType::*;
            word.chars().map(move |char| {
                let maybe_paren = match ptype {
                    Left => {
                        if char == '(' {
                            Ok(LParen)
                        } else {
                            Err(Left)
                        }
                    }
                    Right => {
                        if char == ')' {
                            Ok(RParen)
                        } else {
                            Err(Right)
                        }
                    }
                };
                maybe_paren.expect(&format!(
                    "expected a {:?} parenthesis character",
                    maybe_paren.unwrap_err()
                ))
            })
        }
    }

    fn is_number(word: &str) -> bool {
        word.chars().all(|ch| ch.is_numeric())
    }

    #[derive(Debug, Clone, Copy)]
    enum ParenType {
        Left,
        Right,
    }

    /// # Panics
    ///
    /// If the word is malformed (it doesn't start with parenthesis and there's
    /// parenthesis in the middle of it).
    fn has_paren(word: &str) -> Option<ParenType> {
        if word.starts_with('(') {
            Some(ParenType::Left)
        } else if word.starts_with(')') {
            Some(ParenType::Right)
        }
        // This word doesn't start with parens, so if there's any parenthesis
        // left it's malformed.
        else if word.contains(&['(', ')'][..]) {
            panic!("a word cannot contain parenthesis within it")
        } else {
            None
        }
    }
}

/// This parser will not be helpful for malformed expressions. For example,
/// parenthesis are allowed inside argument names, so things like h(et(((s)grap
/// are fully valid. Don't write that way...
///
/// Things like this are straightforwardly fixable (add a tokenizing step or use
/// pest and a PEG), but I just want to be quick and dirty right now.
///
/// One interesting design decision I could make is to remove from the language
/// the idea of a "nested" operation, and simply compile nestedness into a
/// top-level operation that precedes the current operation, binding it to a
/// result with an auto-generated name, and having the current operation simply
/// reference it by name.
///
/// I honestly like this idea, it opens up something low-level, makes our
/// language's parsed form something we can interpret more like assembly. This
/// might be in keeping with my design goals. Sounds fun to me. Essentially I'm
/// saying that my language doesn't really have nesting, it just has references,
/// but we let you write nested expressions for convenience, and we can output
/// the unrolled form for you to look at.
///
/// NOTE: I've stopped implementation on this to redo construction with a
/// Tokenize -> Parsing construction, and do single pass.
fn nested_parse(code_string: &str) -> Vec<Operation> {
    let mut words = code_string.split_whitespace();
    // This is our results buffer.
    let mut operations: Vec<Operation> = Vec::new();
    let mut parens_bal = 0;
    // A stack of the operations that haven't finished building. We push to this
    // stack every time an operation has subexpressions that we need to parse.
    let mut op_stack: Vec<OpBuilder> = Vec::new();
    let mut current_op = OpBuilder::new();

    // I code this as a loop instead of direct iteration just in case I need to
    // get more complex later and do things like backtracking.
    'main: loop {
        // Get next word and end parsing if there isn't any.
        let word = match words.next() {
            Some(val) => val,
            None => return operations,
        };

        // A word beginning with a paren means it's an operation.
        if let Some(word) = word.strip_prefix('(') {
            parens_bal += 1;
            current_op.name = Some(word.to_string());
            // We want to check if this operation *belongs* inside of another
            // operation. This will be true if the balance of parens is higher
            // than 1.
            if parens_bal > 1 {
                // If it does, then we need to pause completing our parsing of
                // the current operation while we figure out this suboperation.
                op_stack.push(current_op);
                current_op = OpBuilder::new();
            }

            // We have the name, now we need to get the arguments (which
            // themselves might be operations). The loop will repeat.
        }
        // A word ending with a paren means it's the last argument to our
        // current operation, and we can wrap up parsing it. Except we have a
        // difficulty here: what if there's multiple parens stacked next to each
        // other? Well, we can check for that I guess.
        else if let Some(word) = word.strip_suffix(')') {
            parens_bal -= 1;
            // Finish up the current op by adding this as its last argument.
            current_op.arguments.push(ValueOrOperation::Val(
                word.trim_end_matches(')').to_string(),
            ));
            // If this ends us being nested, we have a full complete operation
            // to push to our results.
            if parens_bal == 0 {
                operations.push(current_op.to_op());
                current_op = OpBuilder::new();
                continue 'main;
            }
            // If we're still nested, so let's say our word is `20)` in the expr
            // `(hey (man 10 20) god)`, we've finished the current operation,
            // but haven't gotten a top-level operation yet.
            else {
            }

            // Continue checking for parenthesis
            while let Some(word) = word.strip_suffix(')') {}
            // current_op.arguments.push(word);
        }
        // A word without an open paren is just an argument for an operation
        else {
        }
    }
}

/// Expressions always begin and end with parenthesis.
fn is_expr(word: &str) {}

fn parse_expr<'a>(mut expression: impl Iterator<Item = &'a str>) {}

/// Syntax: (name arguments*).
struct Operation {
    name: String,
    arguments: Vec<ValueOrOperation>,
}

enum ValueOrOperation {
    Val(String),
    Op(Operation),
}

struct OpBuilder {
    name: Option<String>,
    arguments: Vec<ValueOrOperation>,
}

impl OpBuilder {
    fn new() -> Self {
        Self {
            name: None,
            arguments: Vec::new(),
        }
    }
}

impl OpBuilder {
    fn to_op(self) -> Operation {
        Operation {
            name: self.name.unwrap(),
            arguments: self.arguments,
        }
    }
}
