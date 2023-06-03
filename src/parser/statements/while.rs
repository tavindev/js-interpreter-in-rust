use crate::parser::expression::Expression;

use super::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}
