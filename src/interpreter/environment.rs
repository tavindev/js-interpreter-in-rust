use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use crate::parser::value::Value;

use super::functions::{
    implementations::{clock, random},
    native_function::NativeFunction,
};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            enclosing: None,
            values: self.values.clone(),
        }
    }
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            enclosing: None,
            values: HashMap::new(),
        };

        define_native_functions(&mut env);

        env
    }

    pub fn new_enclosing(enclosing: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define<S: Into<String>>(&mut self, name: S, value: Value) {
        let name = name.into();

        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Value {
        if let Some(value) = self.values.get(name) {
            return value.clone();
        }

        if let Some(enclosing) = &self.enclosing {
            let enclosing = enclosing.deref().borrow_mut();

            if enclosing.has(name) {
                let value = enclosing.get(name);
                return value;
            }
        }

        panic!("Undefined variable: {}", name);
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        if let Some(_) = self.values.get(name) {
            self.values.insert(name.to_string(), value);
            return;
        }

        // if enclosing.has(name) {
        //     enclosing.assign(name, value);
        //     return;
        // }

        // Assign to enclosing
        if let Some(enclosing) = &self.enclosing {
            let mut enclosing = enclosing.deref().borrow_mut();

            if enclosing.has(name) {
                enclosing.assign(name, value);
                return;
            }
        }

        panic!("Undefined variable: {}", name);
    }
    pub fn has(&self, name: &str) -> bool {
        if let Some(_) = self.values.get(name) {
            return true;
        }

        if let Some(enclosing) = &self.enclosing {
            let enclosing = enclosing.borrow();

            if enclosing.has(name) {
                return true;
            }
        }

        return false;
    }

    pub fn contents(&self) -> &HashMap<String, Value> {
        &self.values
    }
}

fn define_native_functions(env: &mut Environment) {
    env.define(
        "clock",
        Value::Function(Box::new(NativeFunction::new("clock", vec![], |_, _| {
            return clock();
        }))),
    );

    env.define(
        "random",
        Value::Function(Box::new(NativeFunction::new("random", vec![], |_, _| {
            return random();
        }))),
    );
}
