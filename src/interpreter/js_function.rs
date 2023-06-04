use crate::{
    interpreter::environment::Environment,
    parser::{function::Function, value::Value},
};

use super::{callable::Callable, interpreter::Interpreter};

#[derive(Debug, Clone)]
pub struct JsFunction {
    declaration: Function,
}

impl JsFunction {
    pub fn new(declaration: Function) -> Self {
        Self { declaration }
    }
}

impl Callable for JsFunction {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let globals = interpreter.get_globals();
        let mut environment = Environment::new_enclosing(globals.clone());

        for (parameter, argument) in self
            .declaration
            .parameters
            .iter()
            .zip(arguments.into_iter())
        {
            let ident = parameter.clone();

            environment.define(ident.value(), argument);
        }

        let body = self.declaration.body.clone();

        return interpreter.execute_block(body, environment);
    }
}
