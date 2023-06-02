use std::collections::HashMap;

use crate::parser::value::Value;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosing(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> &Value {
        if let Some(value) = self.values.get(name) {
            return value;
        }

        if let Some(enclosing) = &self.enclosing {
            if enclosing.has(name) {
                return enclosing.get(name);
            }
        }

        panic!("Undefined variable: {}", name);
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        if let Some(_) = self.values.get(name) {
            self.values.insert(name.to_string(), value);
            return;
        }

        if let Some(enclosing) = &mut self.enclosing {
            if enclosing.has(name) {
                enclosing.assign(name, value);
                return;
            }
        }

        panic!("Undefined variable: {}", name);
    }

    pub fn has(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
