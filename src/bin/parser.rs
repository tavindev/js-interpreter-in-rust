use js_interpreter_in_rust::parser::parser::Parser;
use std::io::{self, Write};

pub fn main() {
    let stdin = std::io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let parsed = Parser::new(line).parse();
        println!("{:?}", parsed);
    }
}
