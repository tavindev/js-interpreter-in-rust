use std::{cell::RefCell, rc::Rc};

use js_interpreter_in_rust::{
    interpreter::{environment::Environment, interpreter::Interpreter},
    parser::parser::Parser,
};

fn main() {
    let path = std::env::args().nth(1).expect("missing path argument");
    let source = std::fs::read_to_string(path).expect("failed to read file");
    let mut parser = Parser::new(source);
    let statements = parser.parse();

    let mut environment = Rc::new(RefCell::new(Environment::new()));
    let mut intepreter = Interpreter::new(statements);

    intepreter.run(&mut environment);
}
