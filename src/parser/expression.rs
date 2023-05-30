use super::{operator::Operator, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Value),
    Operator {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
