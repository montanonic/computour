mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use std::io;

pub fn main() {
    println!("Welcome to the Monkey REPL");
    repl::start(io::stdin(), io::stdout());
}
