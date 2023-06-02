use crate::parser::expression::Expression;

use super::{block::BlockStatement, statement::Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: Box<Statement>,
    pub alternative: Option<Box<Statement>>,
}
