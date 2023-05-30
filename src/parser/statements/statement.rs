use super::{block::BlockStatement, r#if::IfStatement, r#let::LetStatement};

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    Block(BlockStatement),
}
