mod calculator;
mod interpreter;
mod parser;
mod understanding_lifetimes;

use parser::Parser;

pub fn main() {
    let example_map_creation = r"
// 
This shows off the ease of adding syntax, with optional commas
and the use of colons in identifiers. mapping is assumed to be a macro,
and will parse this data into a map.
\\
(mapping, yes: god, is: here, and: you?)
";

    let source = r"
(def x (+ 3 4))
(print x)
    ";

    println!("{}", source);

    let parser = Parser::new(source);
    let program = parser.parse();
    println!("{:#?}", program);
    println!("{}", program);

    interpreter::Interpreter::new(&program).interpret();
}
