use crate::parser::{expression::Expression, ident::Ident};

pub struct LetStatement {
    pub name: Ident,
    pub expression: Expression,
}
