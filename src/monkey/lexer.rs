use std::{convert::TryInto, str::FromStr};

use crate::monkey::*;
use token::Token;

#[derive(Debug, Default)]
pub struct Lexer<'input> {
    input: &'input str,
    /// Current position in input (points to current char).
    position: u32,
    /// Current reading position in input (after current char).
    read_position: u32,
    /// Current char under examination. Handled as an ASCII byte.
    ch: u8,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut this = Self {
            input,
            ..Self::default()
        };
        this.read_char();
        this
    }

    /// Gives us the next character and advances our position in the input
    /// string.
    fn read_char(&mut self) {
        // Have we reached the end of input?
        if self.read_position as usize >= self.input.len() {
            // 0 signifies ASCII "NUL", which for us will mean "end of file".
            self.ch = 0;
        }
        // If we haven't reached the end of input...
        else {
            // ...set `ch` to the next character.
            self.ch = self.input.as_bytes()[self.read_position as usize];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Looks at what the next char would be without advancing the iterator.
    fn peek_char(&self) -> Option<char> {
        if self.read_position as usize >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position as usize] as char)
        }
    }

    /// Givens the next lexed token from the input code. Once we reach the end
    /// of the input, None will be returned.
    fn next_token(&mut self) -> Option<Token<'input>> {
        self.skip_whitespace();

        let ch = self.ch as char;
        let token = match ch {
            ';' | ',' | '+' | '-' | '*' | '/' | '<' | '>' | '{' | '}' | '(' | ')' => {
                Token::from_str(&ch.to_string()).unwrap()
            }
            '=' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::NotEQ
                } else {
                    Token::Bang
                }
            }
            '\0' => return None,
            _ => {
                if is_valid_ident_letter(ch) {
                    // Early exit to avoid calling read_char again, as
                    // lookup_ident calls read_char until we fail to match (so
                    // it's already where it should be).
                    return Some(Token::lookup_ident(self.read_identifier()));
                } else if ch.is_ascii_digit() {
                    // Early exit for the same reason as identifiers.
                    return Some(Token::Int(self.read_number().parse().expect(
                        "Expected digits to parse into an i64 value, but parsing failed.",
                    )));
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        Some(token)
    }

    /// From the current position, reads as many characters as possible that are
    /// valid identifier letters.
    fn read_identifier(&mut self) -> &'input str {
        self.read_while(is_valid_ident_letter)
    }

    /// From the current position, reads as many characters as possible that are digits.
    fn read_number(&mut self) -> &'input str {
        self.read_while(|ch| ch.is_ascii_digit())
    }

    fn read_while(&mut self, pred: impl Fn(char) -> bool) -> &'input str {
        let position = self.position as usize;
        while pred(self.ch as char) {
            self.read_char();
        }
        &self.input[position..self.position as usize]
    }

    fn skip_whitespace(&mut self) {
        while (self.ch as char).is_ascii_whitespace() {
            self.read_char();
        }
    }
}

/// Checks that the character is a valid letter for an identifier.
fn is_valid_ident_letter(ch: char) -> bool {
    match ch {
        '_' | '!' => true,
        _ => ch.is_ascii_alphabetic(),
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        use Token::*;

        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10 > 5;

10 == 10;
10 != 9;
";

        let mut l = Lexer::new(input);
        let tests = vec![
            // Line 1
            Let,
            Ident("five"),
            Assign,
            Int(5),
            Semicolon,
            // Line 2
            Let,
            Ident("ten"),
            Assign,
            Int(10),
            Semicolon,
            // Line 4
            Let,
            Ident("add"),
            Assign,
            Function,
            LParen,
            Ident("x"),
            Comma,
            Ident("y"),
            RParen,
            LBrace,
            // Line 5
            Ident("x"),
            Plus,
            Ident("y"),
            Semicolon,
            // Line 6
            RBrace,
            Semicolon,
            // Line 8
            Let,
            Ident("result"),
            Assign,
            Ident("add"),
            LParen,
            Ident("five"),
            Comma,
            Ident("ten"),
            RParen,
            Semicolon,
            // Line 10
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int(5),
            Semicolon,
            // Line 11
            Int(5),
            LT,
            Int(10),
            GT,
            Int(5),
            Semicolon,
            // Line 13
            Int(10),
            EQ,
            Int(10),
            Semicolon,
            // Line 14
            Int(10),
            NotEQ,
            Int(9),
            Semicolon,
        ];

        for (i, test_token) in tests.into_iter().enumerate() {
            assert_eq!(l.next().unwrap(), test_token, "at token #{}", i + 1);
        }
        assert_eq!(l.next(), None, "no more tokens should be left in the lexer");
    }
}
