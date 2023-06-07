use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use js_interpreter_in_rust::{
    interpreter::{environment::Environment, interpreter::Interpreter},
    parser::parser::Parser,
};

fn main() {
    let stdin = std::io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let statements = Parser::new(line).parse();

        let mut environment = Rc::new(RefCell::new(Environment::new()));
        Interpreter::new(statements).run(&mut environment);
    }
}
