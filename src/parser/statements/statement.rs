use super::{r#if::IfStatement, r#let::LetStatement};

pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
}
