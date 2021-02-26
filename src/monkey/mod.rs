mod lexer;
mod token;
pub fn main() {
    println!("{}", token::Token::Comma);
    println!("{}", token::Token::Semicolon);
    println!("{}", (0 as char) as u8 == 0);
}
