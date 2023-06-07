use parser::ident::Ident;

use crate::{callable::Callable, interpreter::Interpreter, value::Value};

#[derive(Clone)]
pub struct NativeFunction {
    name: String,
    arguments: Vec<Ident>,
    function: fn(&mut Interpreter, Vec<Value>) -> Value,
}

impl NativeFunction {
    pub fn new<S: Into<String>>(
        name: S,
        arguments: Vec<Ident>,
        function: fn(&mut Interpreter, Vec<Value>) -> Value,
    ) -> Self {
        Self {
            name: name.into(),
            arguments,
            function,
        }
    }
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
