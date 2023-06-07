use js_interpreter_in_rust::lexer::lexer::{Lexer, Token};
use std::io::{self, Write};

pub fn main() {
    let stdin = std::io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let mut lexer = Lexer::new(line);

        loop {
            let token = lexer.next_token();

            if token == Token::Eof {
                break;
            }

            print!("{:?} ", token);
        }

        println!("");
    }
}
