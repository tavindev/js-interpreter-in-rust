use crate::{
    interpreter::{callable::Callable, environment::Environment, interpreter::Interpreter},
    parser::{ident::Ident, statements::block::BlockStatement, value::Value},
};

#[derive(Debug, Clone)]
pub struct JsFunction {
    ident: Ident,
    parameters: Vec<Ident>,
    body: BlockStatement,
    closure: Environment,
}

impl JsFunction {
    pub fn new(
        ident: Ident,
        parameters: Vec<Ident>,
        body: BlockStatement,
        closure: Environment,
    ) -> Self {
        Self {
            ident,
            parameters,
            body,
            closure,
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
        let mut environment: Environment = Environment::new_enclosing(self.closure.clone()); // TODO: We should pass by reference

        dbg!(&environment);

        for (parameter, argument) in self.parameters.iter().zip(arguments.into_iter()) {
            let ident = parameter.clone();

            environment.define(ident.value(), argument);
        }

        let body = self.body.clone();
        let ret = interpreter.execute_block(body, &mut environment);

        return ret;
    }
}

impl PartialEq for JsFunction {
    fn eq(&self, other: &Self) -> bool {
        return self.ident == other.ident;
    }
}
