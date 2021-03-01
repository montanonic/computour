mod calculator;

use boolinator::Boolinator;
use std::borrow::Cow;

type Str<'a> = Cow<'a, str>;

pub fn main() {
    println!("{:?}", "+32".parse::<i32>());
}

fn parse(source: &str) {
    let tokens = tokenize(source);
}

fn tokenize<'a>(source: &'a str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let source = ensure_each_token_has_whitespace_surrounding(source);
    for word in source.split_whitespace() {
        // Safe because split_whitespace will not produce empty strings.
        let first_char = word.chars().nth(0).unwrap();

        if word.chars().all(|ch| ch.is_alphabetic()) {
            tokens.push(Token::Ident(word.into()));
        }
        // Is a positive or negative number.
        else if (first_char == '-' || first_char == '+' || first_char.is_ascii_digit())
            && word.chars().skip(1).all(|ch| ch.is_ascii_digit())
        {
            tokens.push(Token::Int64(word.parse().unwrap()));
        }
        // First char is alphabetic, and rest of chars are alphanumeric.
        else if first_char.is_alphabetic() && word.chars().skip(1).all(|ch| ch.is_alphanumeric())
        {
            tokens.push(Token::Ident(word.into()));
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
enum Token {
    Ident(String),
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
        let tokens = tokenize(input);
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
