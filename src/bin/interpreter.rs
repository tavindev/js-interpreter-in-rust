use std::io::{self, Write};

use js_interpreter_in_rust::interpreter::interpreter::Interpreter;

fn main() {
    let stdin = std::io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        Interpreter::new(line).run();
    }
}
