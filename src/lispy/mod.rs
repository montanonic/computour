mod calculator;
mod interpreter;
mod parser;
mod understanding_lifetimes;

use parser::Parser;

pub fn main() {
    let source = "(+ 3 (why 9 ten) yes)";
    let parser = Parser::new(source);
    println!("{:?}", parser.parse());
}
