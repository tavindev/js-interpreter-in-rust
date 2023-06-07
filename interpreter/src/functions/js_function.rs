use std::{cell::RefCell, rc::Rc};

use parser::{ident::Ident, statements::block::BlockStatement};

use crate::{callable::Callable, environment::Environment, interpreter::Interpreter, value::Value};

#[derive(Debug, Clone)]
pub struct JsFunction {
    name: String,
    parameters: Vec<Ident>,
    body: BlockStatement,
    // closure: Rc<RefCell<Environment>>,
}

#[allow(dead_code)]
impl JsFunction {
    pub fn new<S: Into<String>>(
        name: S,
        parameters: Vec<Ident>,
        body: BlockStatement,
    ) -> Box<Self> {
        Box::new(Self {
            name: name.into(),
            parameters,
            body,
        })
    }
}

impl Callable for JsFunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn arity(&self) -> usize {
        return self.parameters.len();
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let mut environment = Rc::new(RefCell::new(Environment::new())); // TODO: We should pass by reference

        for (parameter, argument) in self.parameters.iter().zip(arguments.into_iter()) {
            let ident = parameter.clone();

            environment.borrow_mut().define(ident.value(), argument);
        }

        let body = self.body.clone();
        let ret = interpreter.execute_block(body, &mut environment);

        return ret;
    }
}

impl PartialEq for JsFunction {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}
