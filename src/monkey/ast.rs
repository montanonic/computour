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

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'input> {
    Let {
        name: &'input str,
        value: Expression<'input>,
    },
    Return(Expression<'input>),
    Expression(Expression<'input>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'input> {
    Identifier(&'input str),
    IntegerLiteral(i64),
    Prefix {
        operator: Token<'input>,
        right: Box<Expression<'input>>,
    },
}

// pub struct LetStatement<'input> {
//     pub name: &'input str,
//     pub value: Expression,
// }

// pub struct ReturnStatement<'input> {
//     pub value: Expression,
// }
