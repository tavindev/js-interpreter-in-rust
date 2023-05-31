use crate::lexer::lexer::Token;

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Token),
    Operator {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
