use crate::parser::{expression::Expression, ident::Ident};

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: Ident,
    pub expression: Expression,
}
