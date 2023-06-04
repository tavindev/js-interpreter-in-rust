use crate::parser::ident::Ident;

use super::block::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub ident: Ident,
    pub parameters: Vec<Ident>,
    pub body: BlockStatement,
}
