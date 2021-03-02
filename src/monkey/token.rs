use std::{collections::HashMap, str::FromStr};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

use crate::monkey::parser;
use Token::*;
// static KEYWORDS: HashMap<&'static str, Token<'static>> = vec![("fn", Token::Function), ("let", Let)].into_iter().collect();

#[derive(Debug, PartialEq, Eq, Display, EnumString, AsRefStr, IntoStaticStr, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Token<'str> {
    Illegal,

    // Identifiers + literals
    Ident(&'str str),
    Int(i64),

    // Operators
    #[strum(serialize = "=")]
    Assign,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "!")]
    Bang,
    #[strum(serialize = "*")]
    Asterisk,
    #[strum(serialize = "/")]
    Slash,

    #[strum(serialize = "<")]
    LT,
    #[strum(serialize = ">")]
    GT,

    #[strum(serialize = "==")]
    EQ,
    #[strum(serialize = "!=")]
    NotEQ,

    // Delimiters
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    Semicolon,

    #[strum(serialize = "(")]
    LParen,
    #[strum(serialize = ")")]
    RParen,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,

    // Keywords
    #[strum(serialize = "fn")]
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl<'str> Token<'str> {
    /// Looks up to see if a valid identifier string is a keyword, returning the
    /// keyword token, and otherwise returning an identifier. Validation of the string
    /// is expected and not verified.
    pub fn lookup_ident(str: &'str str) -> Self {
        Token::from_str(str).map_or_else(
            |_| Ident(str),
            |token| {
                match token {
                    Function | Let | True | False | If | Else | Return => token,
                    // Token is not a keyword.
                    _ => Ident(str),
                }
            },
        )
    }

    /// Gets a token's precedence. Defaults to Lowest precedence when called on
    /// any token that this is not implemented for.
    pub fn get_precedence(&self) -> parser::Precedence {
        match self {
            EQ => parser::Precedence::Equals,
            NotEQ => parser::Precedence::Equals,
            LT => parser::Precedence::LessOrGreaterThan,
            GT => parser::Precedence::LessOrGreaterThan,
            Plus => parser::Precedence::Sum,
            Minus => parser::Precedence::Sum,
            Slash => parser::Precedence::Product,
            Asterisk => parser::Precedence::Product,
            _ => parser::Precedence::Lowest,
        }
    }
}
