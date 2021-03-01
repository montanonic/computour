mod calculator;

use boolinator::Boolinator;
use std::borrow::Cow;

type Str<'a> = Cow<'a, str>;

pub fn main() {
    println!("{:?}", "+32".parse::<i32>());
}

fn parse(source: &str) {
    let tokens = Tokenizer::new(source).tokenize();
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

            if word.chars().all(|ch| ch.is_alphabetic()) {
                tokens.push(Token::Ident(word));
            }
            // Is a positive or negative number.
            else if (first_char == '-' || first_char == '+' || first_char.is_ascii_digit())
                && word.chars().skip(1).all(|ch| ch.is_ascii_digit())
            {
                tokens.push(Token::Int64(word.parse().unwrap()));
            }
            // First char is alphabetic, and rest of chars are alphanumeric.
            else if first_char.is_alphabetic()
                && word.chars().skip(1).all(|ch| ch.is_alphanumeric())
            {
                tokens.push(Token::Ident(word));
            } else if word == "(" {
                tokens.push(Token::LParen);
            } else if word == ")" {
                tokens.push(Token::RParen);
            } else {
                unimplemented!()
            }
        }
        tokens
    }
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
    Ident(&'a str),
    Int64(i64),
    LParen,
    RParen,
}

/// Expects source to start with an LParen.
fn extract_expression(source_starting_at_expr: Cow<'_, str>) {
    let mut chars = source_starting_at_expr.chars();
    assert_eq!(chars.nth(0), Some('('));

    // chars.
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
            Ident("adam".into()),
            Ident("app11e".into()),
        ];
        for (i, token) in expected.into_iter().enumerate() {
            assert_eq!(token, tokens[i]);
        }
    }
}
