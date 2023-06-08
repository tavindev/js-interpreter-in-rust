use core::fmt;

use crate::{ident::Ident, statements::block::BlockStatement};

#[derive(Clone, PartialEq)]
pub enum Value {
    Function {
        ident: Option<Ident>,
        params: Vec<Ident>,
        body: BlockStatement,
    },
    String(String),
    Number(String),
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
                ident,
                params: _,
                body: _,
            } => {
                if let Some(ident) = ident {
                    write!(f, "<function {}>", ident.value())
                } else {
                    write!(f, "<anonymous function>")
                }
            }
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

    pub fn function(ident: Option<Ident>, params: Vec<Ident>, body: BlockStatement) -> Self {
        Value::Function {
            ident,
            params,
            body,
        }
    }
}
