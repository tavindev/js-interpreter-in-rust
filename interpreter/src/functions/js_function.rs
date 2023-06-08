use std::rc::Rc;

use parser::{ident::Ident, statements::block::BlockStatement};

use crate::{callable::Callable, environment::Environment, interpreter::Interpreter, value::Value};

#[derive(Debug, Clone)]
pub struct JsFunction {
    name: Option<String>,
    parameters: Vec<Ident>,
    body: BlockStatement,
    closure: Rc<Environment>,
}

#[allow(dead_code)]
impl JsFunction {
    pub fn new<S: Into<String>>(
        name: Option<S>,
        parameters: Vec<Ident>,
        body: BlockStatement,
        closure: Rc<Environment>,
    ) -> Box<Self> {
        Box::new(Self {
            name: name.map(|s| s.into()),
            parameters,
            body,
            closure,
        })
    }
}

impl Callable for JsFunction {
    fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "<anonymous function>".to_string())
    }

    fn set_name(&mut self, _: String) {
        unimplemented!()
    }

    fn arity(&self) -> usize {
        return self.parameters.len();
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let environment = Rc::new(Environment::new_enclosing(&self.closure)); // TODO: We should pass by reference

        for (parameter, argument) in self.parameters.iter().zip(arguments.into_iter()) {
            let ident = parameter.clone();

            environment.define(ident.value(), argument);
        }

        let body = self.body.clone();
        let ret = interpreter.execute_block(body, &environment);

        return ret;
    }
}

impl PartialEq for JsFunction {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
