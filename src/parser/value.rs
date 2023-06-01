#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Ident(String), // TODO: Remove this
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(number) => *number != 0.0,
            Value::Bool(bool) => *bool,
            Value::Null => false,
            _ => true,
        }
    }

    pub fn sum(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::String(left), Value::String(right)) => {
                Value::String(format!("{}{}", left, right))
            }
            _ => unimplemented!(),
        }
    }

    pub fn sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
            _ => unimplemented!(),
        }
    }

    pub fn mult(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
            _ => unimplemented!(),
        }
    }

    pub fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left / right),
            _ => unimplemented!(),
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            Value::Number(number) => *number,
            _ => panic!("Cannot convert {:?} to number", self),
        }
    }
}
