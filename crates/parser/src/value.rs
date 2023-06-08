use core::fmt;

use crate::{ident::Ident, statements::block::BlockStatement};

#[derive(Clone, PartialEq)]
pub enum ParserValue {
    String(String),
    Number(String),
    Bool(bool),
    Null,
    Function {
        ident: Option<Ident>,
        params: Vec<Ident>,
        body: BlockStatement,
    },
}

impl fmt::Debug for ParserValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserValue::String(string) => write!(f, "{}", string),
            ParserValue::Number(number) => write!(f, "{}", number),
            ParserValue::Bool(bool) => write!(f, "{}", bool),
            ParserValue::Null => write!(f, "null"),
            ParserValue::Function {
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

impl ParserValue {
    pub fn number<T: Into<String>>(number: T) -> Self {
        ParserValue::Number(number.into())
    }

    pub fn string<T: Into<String>>(string: T) -> Self {
        ParserValue::String(string.into())
    }

    pub fn bool(boolean: bool) -> Self {
        ParserValue::Bool(boolean)
    }

    pub fn null() -> Self {
        ParserValue::Null
    }

    pub fn function(ident: Option<Ident>, params: Vec<Ident>, body: BlockStatement) -> Self {
        ParserValue::Function {
            ident,
            params,
            body,
        }
    }
}
