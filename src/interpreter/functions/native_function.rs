use crate::{
    interpreter::{callable::Callable, interpreter::Interpreter},
    parser::{ident::Ident, value::Value},
};

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arguments: Vec<Ident>,
    pub function: fn(&mut Interpreter, Vec<Value>) -> Value,
}

impl Callable for NativeFunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Value {
        (self.function)(_interpreter, _arguments)
    }

    fn arity(&self) -> usize {
        self.arguments.len()
    }
}
