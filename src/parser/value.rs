use core::fmt;

use crate::interpreter::{callable::Callable, functions::js_function::JsFunction};

use super::{ident::Ident, statements::block::BlockStatement};

#[derive(Clone)]
pub enum Value {
    Function(Box<dyn Callable>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(number), Value::Number(other_number)) => number == other_number,
            (Value::String(string), Value::String(other_string)) => string == other_string,
            (Value::Bool(bool), Value::Bool(other_bool)) => bool == other_bool,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "{}", string),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Null => write!(f, "null"),
            Value::Function(function) => write!(f, "<function {}>", function.name()),
        }
    }
}

impl Value {
    pub fn number<T: Into<f64>>(number: T) -> Self {
        Value::Number(number.into())
    }

    pub fn string<T: Into<String>>(string: T) -> Self {
        Value::String(string.into())
    }

    pub fn bool<T: Into<bool>>(bool: T) -> Self {
        Value::Bool(bool.into())
    }

    pub fn null() -> Self {
        Value::Null
    }

    pub fn function(ident: Ident, parameters: Vec<Ident>, body: BlockStatement) -> Self {
        Value::Function(Box::new(JsFunction::new(ident, parameters, body)))
    }

    pub fn to_number(&self) -> f64 {
        match self {
            Value::Number(number) => *number,
            _ => panic!("Cannot convert {:?} to number", self),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(number) => *number != 0.0,
            Value::Bool(bool) => *bool,
            Value::Null => false,
            _ => true,
        }
    }

    pub fn not(&self) -> Value {
        Value::Bool(!self.is_truthy())
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

    pub fn gt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Bool(left > right),
            (Value::String(left), Value::String(right)) => Value::Bool(left > right),
            _ => unimplemented!(),
        }
    }

    pub fn lt(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Bool(left < right),
            (Value::String(left), Value::String(right)) => Value::Bool(left < right),
            _ => unimplemented!(),
        }
    }

    pub fn gte(&self, other: &Value) -> Value {
        return self.lt(other).not();
    }

    pub fn lte(&self, other: &Value) -> Value {
        return self.gt(other).not();
    }

    pub fn eq(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Bool(left == right),
            (Value::String(left), Value::String(right)) => Value::Bool(left == right),
            (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
            (Value::Null, Value::Null) => Value::Bool(true),
            (Value::Null, _) => Value::Bool(false),
            _ => unimplemented!(),
        }
    }

    pub fn neq(&self, other: &Value) -> Value {
        return self.eq(other).not();
    }

    pub fn and(&self, other: &Value) -> Value {
        return Value::Bool(self.is_truthy() && other.is_truthy());
    }

    pub fn or(&self, other: &Value) -> Value {
        return Value::Bool(self.is_truthy() || other.is_truthy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truthy() {
        assert_eq!(Value::Number(0.0).is_truthy(), false);
        assert_eq!(Value::Number(1.0).is_truthy(), true);
        assert_eq!(Value::Bool(false).is_truthy(), false);
        assert_eq!(Value::Bool(true).is_truthy(), true);
        assert_eq!(Value::Null.is_truthy(), false);
        assert_eq!(Value::String("".to_string()).is_truthy(), true);
        assert_eq!(Value::String("foo".to_string()).is_truthy(), true);
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            Value::Number(1.0).sum(&Value::Number(2.0)),
            Value::Number(3.0)
        );
        assert_eq!(
            Value::String("foo".to_string()).sum(&Value::String("bar".to_string())),
            Value::String("foobar".to_string())
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Value::Number(1.0).sub(&Value::Number(2.0)),
            Value::Number(-1.0)
        );
    }

    #[test]
    fn test_mult() {
        assert_eq!(
            Value::Number(1.0).mult(&Value::Number(2.0)),
            Value::Number(2.0)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Value::Number(1.0).div(&Value::Number(2.0)),
            Value::Number(0.5)
        );
    }

    #[test]
    fn test_to_number() {
        assert_eq!(Value::Number(1.0).to_number(), 1.0);
    }

    #[test]
    fn test_gt() {
        assert_eq!(
            Value::Number(1.0).gt(&Value::Number(2.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::Number(2.0).gt(&Value::Number(1.0)),
            Value::Bool(true)
        );
        assert_eq!(
            Value::String("foo".to_string()).gt(&Value::String("bar".to_string())),
            Value::Bool(true)
        );
        assert_eq!(
            Value::String("bar".to_string()).gt(&Value::String("foo".to_string())),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_lt() {
        assert_eq!(
            Value::Number(1.0).lt(&Value::Number(2.0)),
            Value::Bool(true)
        );
        assert_eq!(
            Value::Number(2.0).lt(&Value::Number(1.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("foo".to_string()).lt(&Value::String("bar".to_string())),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("bar".to_string()).lt(&Value::String("foo".to_string())),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_gte() {
        assert_eq!(
            Value::Number(1.0).gte(&Value::Number(2.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::Number(2.0).gte(&Value::Number(1.0)),
            Value::Bool(true)
        );
        assert_eq!(
            Value::String("foo".to_string()).gte(&Value::String("bar".to_string())),
            Value::Bool(true)
        );
        assert_eq!(
            Value::String("bar".to_string()).gte(&Value::String("foo".to_string())),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_lte() {
        assert_eq!(
            Value::Number(1.0).lte(&Value::Number(2.0)),
            Value::Bool(true)
        );
        assert_eq!(
            Value::Number(2.0).lte(&Value::Number(1.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("foo".to_string()).lte(&Value::String("bar".to_string())),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("bar".to_string()).lte(&Value::String("foo".to_string())),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            Value::Number(1.0).eq(&Value::Number(2.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::Number(2.0).eq(&Value::Number(1.0)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("foo".to_string()).eq(&Value::String("bar".to_string())),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("bar".to_string()).eq(&Value::String("foo".to_string())),
            Value::Bool(false)
        );
        assert_eq!(Value::Null.eq(&Value::Null), Value::Bool(true));
        assert_eq!(Value::Null.eq(&Value::Number(1.0)), Value::Bool(false));
    }
}
