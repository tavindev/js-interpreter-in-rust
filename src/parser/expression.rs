use super::{operator::Operator, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Grouping(Box<Expression>),
    Literal(Value),
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate(&self) -> Value {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate();
                let right = right.evaluate();

                match operator {
                    Operator::Plus => left.sum(&right),
                    Operator::Minus => left.sub(&right),
                    Operator::Asterisk => left.mult(&right),
                    Operator::Slash => left.div(&right),
                    _ => unimplemented!(),
                }
            }
            Expression::Grouping(expression) => expression.evaluate(),
            Expression::Literal(value) => value.clone(),
            Expression::Unary { operator, right } => {
                let right = right.evaluate();

                match operator {
                    Operator::Minus => Value::Number(-right.to_number()),
                    Operator::Bang => Value::Bool(!right.is_truthy()),
                    _ => unimplemented!(),
                }
            }
        }
    }
}
