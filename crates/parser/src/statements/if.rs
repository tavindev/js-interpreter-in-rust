use crate::expression::Expression;

use super::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: Box<Statement>,
    pub alternative: Option<Box<Statement>>,
}
