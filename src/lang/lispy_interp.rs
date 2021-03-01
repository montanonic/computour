//! Curious to try an imperative, unstructured implementation and without lexing.

use std::mem;
use tokens::TokenStream;

pub fn run() {
    let code = "
Yes!
";
    println!("{:?}", code);
    interpret(code);
}

fn interpret(code: &str) {}

mod tokens {
    use std::str::Chars;

    use self::Token::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Token {
        LParen,
        RParen,
        /// An alphanumerical word.
        Word(String),
        /// Currently only support integers.
        Number(String),
        Newline,
    }

    pub struct TokenStream<'code> {
        /// Position in our `chars` vector.
        pos: usize,
        chars: Vec<char>,
        /// Lazily feeds into the chars vector.
        char_iter: Chars<'code>,
    }

    impl<'code> TokenStream<'code> {
        pub fn new(code: &'code str) -> Self {
            Self {
                // The vector is potentially overallocated as code.len() >=
                // code.chars().collect().len() (because UTF-8 is
                // variable-length, so multiple bytes might correspond with a single
                // char).
                chars: Vec::with_capacity(code.len()),
                char_iter: code.chars(),
                pos: 0,
            }
        }

        fn next_token(&mut self) -> Option<Token> {
            let mut char = self.next_char()?;

            // Skip whitespace.
            while char.is_whitespace() {
                char = self.next_char()?;
            }

            let token = match char {
                '(' => LParen,
                ')' => RParen,
                '\n' => Newline,
                _ => {
                    if char.is_numeric() {
                        Number(self.chars_into_string(char, char::is_numeric, true))
                    }
                    // Now that we've matched on numbers, we can assume the rest
                    // are alphanumeric chars that don't start with a number.
                    else if char.is_alphanumeric() {
                        Word(self.chars_into_string(char, char::is_alphanumeric, true))
                    } else {
                        panic!("we should never get to this point!")
                    }
                }
            };
            Some(token)
        }

        /// Applies the predicate on the character, advancing the character
        /// iterator and constructing a String for each character that matches.
        /// Once the predicate fails or we run out of characters, we return the
        /// string up to the current point. Note that if the predicate
        /// immediately fails, you'll get an empty string.
        ///
        /// `expect_whitespace` checks to ensure that the end of the input has
        /// whitespace. This is usually what you want here.
        fn chars_into_string(
            &mut self,
            mut char: char,
            pred: impl Fn(char) -> bool,
            expect_whitespace: bool,
        ) -> String {
            let mut vec = Vec::new();
            while pred(char) {
                vec.push(char);
                char = match self.next_char() {
                    Some(c) => c,
                    None => break,
                }
            }
            if expect_whitespace {
                Self::expect_whitespace(char);
            }
            vec.into_iter().collect()
        }

        /// Asserts that the given character is whitespace. Useful for ensuring
        /// mutli-character tokens are correctly formed.
        fn expect_whitespace(char: char) {
            assert!(char.is_whitespace(), "expected whitespace")
        }

        fn next_char(&mut self) -> Option<char> {
            // Check to see if we need to advance more chars into our vector.
            if self.pos >= self.chars.len() {
                self.chars.push(self.char_iter.next()?);
            }

            let char = self.chars[self.pos];
            self.pos += 1;
            Some(char)
        }
    }

    impl Iterator for TokenStream<'_> {
        type Item = Token;
        fn next(&mut self) -> Option<Self::Item> {
            self.next_token()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "broken and not working on for now"]
    fn token_stream_works() {
        use tokens::Token::{self, *};
        let code = "(hey 123 you2 (yes))";
        let tokens: Vec<Token> = TokenStream::new(code).collect();
        assert_eq!(
            &[
                LParen,
                Word("hey".into()),
                Number("123".into()),
                Word("you2".into()),
                LParen,
                Word("yes".into()),
                RParen,
                RParen,
            ],
            &tokens.as_slice()
        );
    }
}
