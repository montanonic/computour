use boolinator::Boolinator;
use std::mem;

use crate::monkey::{
    ast::{self, Statement},
    lexer::Lexer,
    token::Token,
};

use super::lexer;

struct Parser<'input> {
    lexer: Lexer<'input>,
    curr_token: Option<Token<'input>>,
    peek_token: Option<Token<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer::new(input);
        let curr_token = lexer.next();
        let peek_token = lexer.next();

        Self {
            lexer,
            curr_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> ast::Program<'input> {
        let mut program = ast::Program::new();

        while let Some(curr_token) = self.curr_token {
            let stmt = self.parse_statement();
            stmt.map(|s| program.statements.push(s));
            self.next_token();
        }

        program
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    fn parse_statement(&mut self) -> Option<Statement<'input>> {
        self.curr_token.and_then(|token| match token {
            Token::Let => self.parse_let_statement().map(Statement::Let),
            _ => None,
        })
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement<'input>> {
        let name = self.expect_peek(|t| match t {
            Token::Ident(name) => Some(name),
            _ => None,
        })?;

        self.expect_peek(|t| matches!(t, Token::Assign).as_option())?;

        // TODO: We're skipping the expressions until we encounter a
        // semicolon.
        while !matches!(self.curr_token, Some(Token::Semicolon)) {
            self.next_token();
        }

        Some(ast::LetStatement {
            name,
            value: ast::Expression,
        })
    }

    /// Runs the given function on the next token, advancing to the next token
    /// if the function returns Some(T), returning None otherwise. If there is
    /// no next token, returns None.
    fn expect_peek<T>(&mut self, func: impl Fn(Token<'input>) -> Option<T>) -> Option<T> {
        // If there's a next token,
        self.peek_token.and_then(|pt| {
            // see if the function works,
            func(pt).map(|val| {
                // advancing to the next token if it does,
                self.next_token();
                // and returning its value.
                val
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{LetStatement, Statement};

    #[test]
    fn test_let_statement() {
        use Token::Int;
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

        let mut parser = Parser::new(input);

        let program = parser.parse_program();
        // assert!(program.is_some());
        assert_eq!(program.statements.len(), 3);

        let names = vec!["x", "y", "foobar"];
        let values = vec![Int(5), Int(10), Int(838383)];

        for (i, statement) in program.statements.into_iter().enumerate() {
            if let Statement::Let(LetStatement { name, value, .. }) = statement {
                assert_eq!(name, names[i]);
                assert_eq!(value, values[i]);
            } else {
                assert!(false);
            }
        }
    }
}
