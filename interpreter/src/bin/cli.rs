use std::rc::Rc;

use interpreter::{environment::Environment, interpreter::Interpreter};
use parser::parser::Parser;

fn main() {
    let path = std::env::args().nth(1).expect("missing path argument");
    let source = std::fs::read_to_string(path).expect("failed to read file");
    let mut parser = Parser::new(source);
    let statements = parser.parse();

    let environment = Rc::new(Environment::new());
    let mut intepreter = Interpreter::new(statements);

    intepreter.run(&environment);
}
