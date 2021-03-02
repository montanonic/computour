use std::fmt::{self, Display};

use crate::monkey;
use monkey::token::Token;

#[derive(Debug)]
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

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {} = {};", name, value),
            Statement::Return(expr) => write!(f, "return {};", expr),
            Statement::Expression(expr) => write!(f, "{};", expr),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'input> {
    Identifier(&'input str),
    IntegerLiteral(i64),
    Prefix {
        operator: Token<'input>,
        right: Box<Expression<'input>>,
    },
    Infix {
        left: Box<Expression<'input>>,
        operator: Token<'input>,
        right: Box<Expression<'input>>,
    },
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(str) => write!(f, "{}", str),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
        }
    }
}

// pub struct LetStatement<'input> {
//     pub name: &'input str,
//     pub value: Expression,
// }

// pub struct ReturnStatement<'input> {
//     pub value: Expression,
// }
