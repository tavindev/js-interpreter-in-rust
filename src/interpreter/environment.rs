use std::collections::HashMap;

use crate::parser::value::Value;

use super::functions::{
    implementations::{clock, random},
    native_function::NativeFunction,
};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
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

    pub fn new_enclosing(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define<S: Into<String>>(&mut self, name: S, value: Value) {
        let name = name.into();

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

    pub fn contents(&self) -> &HashMap<String, Value> {
        &self.values
    }
}

fn define_native_functions(env: &mut Environment) {
    env.define(
        "clock",
        Value::Function(Box::new(NativeFunction {
            name: "clock".to_string(),
            arguments: vec![],
            function: |_, _| {
                return clock();
            },
        })),
    );

    env.define(
        "random",
        Value::Function(Box::new(NativeFunction {
            name: "random".to_string(),
            arguments: vec![],
            function: |_, _| {
                return random();
            },
        })),
    );
}
