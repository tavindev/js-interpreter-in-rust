use crate::{
    interpreter::{callable::Callable, environment::Environment, interpreter::Interpreter},
    parser::{ident::Ident, statements::block::BlockStatement, value::Value},
};

#[derive(Debug, Clone)]
pub struct JsFunction {
    pub ident: Ident,
    pub parameters: Vec<Ident>,
    pub body: BlockStatement,
}

impl JsFunction {
    pub fn new(ident: Ident, parameters: Vec<Ident>, body: BlockStatement) -> Self {
        Self {
            ident,
            parameters,
            body,
        }
    }
}

impl Callable for JsFunction {
    fn name(&self) -> String {
        let ident = self.ident.clone();

        return ident.value();
    }

    fn arity(&self) -> usize {
        return self.parameters.len();
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let globals = interpreter.get_globals();
        let mut environment = Environment::new_enclosing(globals.clone());

        for (parameter, argument) in self.parameters.iter().zip(arguments.into_iter()) {
            let ident = parameter.clone();

            environment.define(ident.value(), argument);
        }

        let body = self.body.clone();

        return interpreter.execute_block(body, environment);
    }
}

impl PartialEq for JsFunction {
    fn eq(&self, other: &Self) -> bool {
        return self.ident == other.ident;
    }
}
