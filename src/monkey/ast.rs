use crate::monkey;
use monkey::token::Token;

pub struct Program<'input> {
    pub(crate) statements: Vec<Statement<'input>>,
}

impl Program<'_> {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

pub enum Statement<'token> {
    Let(LetStatement<'token>),
}

#[derive(Debug, PartialEq)]
pub struct Expression;

pub struct LetStatement<'token> {
    pub name: &'token str,
    pub value: Expression,
}
