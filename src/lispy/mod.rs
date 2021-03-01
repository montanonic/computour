mod calculator;

use boolinator::Boolinator;
use std::{borrow::Cow, str::pattern::Pattern};

type Str<'a> = Cow<'a, str>;

pub fn main() {
    println!("{:?}", '+'.is_contained_in("+-123"));
}

fn parse(source: &str) {
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        use Token::*;
        // match token {}
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
                tokens.push(Token::Ident(word));
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
    Ident(&'a str),
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
            Ident("adam".into()),
            Ident("app11e".into()),
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
        let tokens = vec![LParen, RParen, LParen];
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens[0..2]);

        let tokenizer = Tokenizer::new("(+ (- 3 4) 5 6)");
        let tokens = tokenizer.tokenize();
        assert_eq!(extract_expression(&tokens).unwrap(), &tokens);
    }
}
