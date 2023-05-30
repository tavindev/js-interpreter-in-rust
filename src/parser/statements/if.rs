use crate::parser::expression::Expression;

use super::block::BlockStatement;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
