use super::{ident::Ident, statements::block::BlockStatement};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub ident: Ident,
    pub parameters: Vec<Ident>,
    pub body: BlockStatement,
}
