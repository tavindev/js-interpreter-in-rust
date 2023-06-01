use crate::parser::{expression::Expression, ident::Ident};

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub ident: Ident,
    pub expression: Option<Expression>,
}
