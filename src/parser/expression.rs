use super::{operator::Operator, value::Value};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Value),
    Operator {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
