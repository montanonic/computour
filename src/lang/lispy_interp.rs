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

    type Tokens<'code> = Vec<Token<'code>>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Token<'code> {
        LParen,
        RParen,
        /// An alphanumerical word.
        Word(&'code str),
        /// Currently only support integers.
        Number(&'code str),
        Newline,
    }

    pub struct TokenStream<'code> {
        /// Position in our `chars` vector.
        pos: usize,
        /// Separate pointer for peeking ahead.
        peak_pos: usize,
        /// Current character.
        char: char,
        chars: Vec<char>,
        /// Lazily feeds into the chars vector.
        char_iter: Chars<'code>,
        code: &'code str,
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
                peak_pos: 0,
                char: 0 as char,
                code
            }
        }

        fn next_token(&mut self) -> Option<Token<'code>> {
            self.next_char()?;

            // Skip whitespace.
            while self.char.is_whitespace() {
                self.next_char()?;
            }

            let token = match self.char {
                '(' => LParen,
                ')' => RParen,
                '\n' => Newline,
                ch => {
                    if ch.is_numeric() {
                        while self.char.
                    }
                },
            };
            Some(token)
        }

        // /// The position of the current char within our stream. Requires
        // /// next_char to be called at least once.
        // fn pos(&self) -> usize {
        //     self.chars.len() - 1
        // }

        fn next_char(&mut self) -> Option<()> {
            // Check to see if we need to advance more chars into our vector.
            if self.pos >= self.chars.len() {
                self.chars.push(self.char_iter.next()?);
            }

            self.char = self.chars[self.pos];
            self.pos += 1;
            self.peak_pos = self.pos;
            Some(())
        }

        /// Peeks the next char, advancing the peek pointer so that this can be
        /// called multiple times in a row. Calling `next_char` will reset the
        /// peek.
        fn peek_next(&mut self) {
            // Check to see if we need to advance more chars into our vector.
            if self.pos >= self.chars.len() {
                self.chars.push(self.char_iter.next()?);
            }

            if self.peak_pos < self.chars.len() {
                // We have extra chars available and don't need to read more.
                self.char = self.chars[self.peak_pos];
                Some(())
            } else {
                // We need to read more chars.
                self.char_iter.next().map(|ch| {
                    self.chars.push(ch);
                })
            }
        }
    }

    impl<'code> Iterator for TokenStream<'code> {
        type Item = Token<'code>;
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
    fn token_stream_works() {
        use tokens::Token::{self, *};
        let code = "(hey 123 you2 (yes))";
        let tokens: Vec<Token<'_>> = TokenStream::new(code).collect();
        assert_eq!(
            &[
                LParen,
                Word("hey"),
                Number("123"),
                Word("you2"),
                LParen,
                Word("yes"),
                RParen,
                RParen,
            ],
            &tokens.as_slice()
        );
    }
}
