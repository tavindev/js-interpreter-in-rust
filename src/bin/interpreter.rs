use std::io::{self, Write};

use js_interpreter_in_rust::{interpreter::interpreter::Interpreter, parser::parser::Parser};

fn main() {
    let stdin = std::io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let statements = Parser::new(line).parse();

        Interpreter::new(statements).run();
    }
}
