use crate::parser::parser::Parser;

pub mod interpreter;
pub mod lexer;
pub mod parser;

pub fn main() {
    let io = std::io::stdin();

    loop {
        let mut line = String::new();
        io.read_line(&mut line).unwrap();

        let parsed = Parser::new(line).parse();
        println!("{:?}", parsed);
    }
}
