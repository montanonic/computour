mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use std::io;

pub fn main() {
    let mut parser = parser::Parser::new("3 + 4 * 5 == 3 * 1 + 4 * 5");
    let parsed = parser.parse_program();
    println!("{:?}", parsed);
    for statement in parsed.statements {
        println!("{}", statement);
    }

    // println!("Welcome to the Monkey REPL");
    // repl::start(io::stdin(), io::stdout());
}
