use boolinator::Boolinator;
use std::mem;

use crate::monkey::{
    ast::{self, Expression, Statement},
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
            let maybe_statement = self.parse_statement();
            match maybe_statement {
                Some(statement) => program.statements.push(statement),
                None => panic!(format!(
                    "Failed to parse at token: {:?}. Next token is: {:?}",
                    curr_token, self.peek_token
                )),
            }
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
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement<'input>> {
        let expression = self
            .parse_expression(Precedence::Lowest)
            .expect("Expected expression to parse");

        // Check for optional semicolon following the expression, advancing it
        // to the current token if so.
        self.peek_token.map(|token| {
            if token == Token::Semicolon {
                self.next_token();
            }
        });

        Some(Statement::Expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression<'input>> {
        self.curr_token.and_then(|token| match precedence {
            _ => self.parse_prefix_token(token),
        })
    }

    fn parse_let_statement(&mut self) -> Option<Statement<'input>> {
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

        Some(Statement::Let {
            name,
            value: todo!(),
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement<'input>> {
        (self.curr_token == Some(Token::Return)).as_option()?;
        self.next_token();
        // TODO: We're skipping the expressions until we encounter a
        // semicolon.
        while !matches!(self.curr_token, Some(Token::Semicolon)) {
            self.next_token();
        }

        Some(Statement::Return(todo!()))
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

    /// Parses the given (prefix) token into an expression.
    fn parse_prefix_token(&mut self, token: Token<'input>) -> Option<Expression<'input>> {
        match token {
            Token::Ident(val) => Some(Expression::Identifier(val)),
            Token::Int(val) => Some(Expression::IntegerLiteral(val)),
            operator @ Token::Bang => self.parse_prefix_expression(operator),
            operator @ Token::Minus => self.parse_prefix_expression(operator),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self, operator: Token<'input>) -> Option<Expression<'input>> {
        self.next_token();

        self.parse_expression(Precedence::Prefix)
            .map(|right| Expression::Prefix {
                operator,
                right: Box::new(right),
            })
    }

    /// Parses the given (infix) token into an expression.
    fn parse_infix_token(
        token: Token<'input>,
        left_side: Expression<'input>,
    ) -> Option<Expression<'input>> {
        match token {
            _ => None,
        }
    }
}

/// The order defined here is implicitly the order of precedence. Adjusting
/// these lines adjusts precedence.
#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,            // ==
    LessOrGreaterThan, // > or <
    Sum,               // +
    Product,           // *
    Prefix,            // -x or !x
    Call,              // my_function(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Expression, Statement};

    #[test]
    fn test_let_statement() {
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
        use Expression::IntegerLiteral as IntLit;
        let values = vec![IntLit(5), IntLit(10), IntLit(838383)];

        for (i, statement) in program.statements.into_iter().enumerate() {
            if let Statement::Let { name, value } = statement {
                assert_eq!(name, names[i]);
                assert_eq!(value, values[i]);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let mut parser = Parser::new(input);

        let pgm = parser.parse_program();
        let statement = &pgm.statements[0];

        assert!(matches!(
            statement,
            &Statement::Expression(Expression::Identifier("foobar"))
        ))
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let mut parser = Parser::new(input);

        let pgm = parser.parse_program();
        let statement = &pgm.statements[0];

        assert!(matches!(
            statement,
            &Statement::Expression(Expression::IntegerLiteral(5))
        ))
    }

    #[test]
    fn test_prefix_expressions() {
        let input = "!5; -15;";
        let mut parser = Parser::new(input);

        let pgm = parser.parse_program();
        let statement1 = &pgm.statements[0];
        let statement2 = &pgm.statements[1];

        assert!(matches!(
            statement1,
            &Statement::Expression(Expression::Prefix {
                operator: Token::Bang,
                right: box Expression::IntegerLiteral(5),
            })
        ));
        assert!(matches!(
            statement2,
            &Statement::Expression(Expression::Prefix {
                operator: Token::Minus,
                right: box Expression::IntegerLiteral(15),
            })
        ));
    }
}
