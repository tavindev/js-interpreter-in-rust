use core::fmt;

use crate::{ident::Ident, statements::statement::Statement};

#[derive(Clone, PartialEq)]
pub enum Value {
    Function {
        arguments: Vec<Ident>,
        body: Vec<Statement>,
    },
    Number(String),
    String(String),
    Bool(bool),
    Null,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(string) => write!(f, "{}", string),
            Value::Number(number) => write!(f, "{}", number),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Null => write!(f, "null"),
            Value::Function {
                arguments: _,
                body: _,
            } => write!(f, "<anonymous function>"),
        }
    }
}

impl Value {
    pub fn number<T: Into<String>>(number: T) -> Self {
        Value::Number(number.into())
    }

    pub fn string<T: Into<String>>(string: T) -> Self {
        Value::String(string.into())
    }

    pub fn bool(boolean: bool) -> Self {
        Value::Bool(boolean)
    }

    pub fn null() -> Self {
        Value::Null
    }

    pub fn function(arguments: Vec<Ident>, body: Vec<Statement>) -> Self {
        Value::Function { arguments, body }
    }
}
