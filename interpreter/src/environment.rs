use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

use super::functions::{
    implementations::{clock, random},
    native_function::NativeFunction,
};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: RefCell<HashMap<String, Value>>,
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
            values: RefCell::new(HashMap::new()),
        };

        define_native_functions(&mut env);

        env
    }

    pub fn new_enclosing(enclosing: &Rc<Environment>) -> Environment {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn define<S: Into<String>>(&self, name: S, value: Value) {
        let name = name.into();

        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Value {
        if let Some(value) = self.values.borrow().get(name) {
            return value.clone();
        }

        if let Some(enclosing) = &self.enclosing {
            if enclosing.has(name) {
                return enclosing.get(name);
            }
        }

        panic!("Undefined variable: {}", name);
    }

    pub fn assign(&self, name: &str, value: Value) {
        let mut values = self.values.borrow_mut();

        if let Some(_) = values.get(name) {
            values.insert(name.to_string(), value);
            return;
        }

        if let Some(enclosing) = &self.enclosing {
            if enclosing.has(name) {
                enclosing.assign(name, value);
                return;
            }
        }

        panic!("Undefined variable: {}", name);
    }
    pub fn has(&self, name: &str) -> bool {
        if let Some(_) = self.values.borrow().get(name) {
            return true;
        }

        if let Some(enclosing) = &self.enclosing {
            if enclosing.has(name) {
                return true;
            }
        }

        return false;
    }

    pub fn contents(&self) -> HashMap<String, Value> {
        return self.values.borrow().clone();
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::value::Value;

    use super::Environment;

    #[test]
    fn enclosing() {
        let outer = Rc::new(Environment::new());
        let inner = Environment::new_enclosing(&outer);

        outer.define("a", Value::Number(1.0));

        assert_eq!(inner.get("a"), Value::Number(1.0));
    }
}
