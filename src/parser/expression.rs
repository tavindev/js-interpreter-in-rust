use super::{ident::Ident, operator::Operator, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(Ident),
    Grouping(Box<Expression>),
    Literal(Value),
    Assignement {
        ident: Ident,
        value: Box<Expression>,
    },
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
    pub fn grouping(expression: Expression) -> Expression {
        Expression::Grouping(Box::new(expression))
    }

    pub fn literal(value: Value) -> Expression {
        Expression::Literal(value)
    }

    pub fn unary(operator: Operator, right: Expression) -> Expression {
        Expression::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn binary(left: Expression, operator: Operator, right: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn assignement(ident: Ident, value: Expression) -> Expression {
        Expression::Assignement {
            ident,
            value: Box::new(value),
        }
    }

    pub fn evaluate(&self) -> Value {
        match self {
            Expression::Assignement { ident: _, value } => value.evaluate(),
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
                    Operator::GreaterThan => left.gt(&right),
                    Operator::GreaterThanOrEqual => left.gte(&right),
                    Operator::LessThan => left.lt(&right),
                    Operator::LessThanOrEqual => left.lte(&right),
                    Operator::Equal => left.eq(&right),
                    Operator::NotEqual => left.neq(&right),
                    Operator::And => left.and(&right),
                    Operator::Or => left.or(&right),
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
            Expression::Variable(ident) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Literal(Value::Number(1.0))),
            operator: Operator::Plus,
            right: Box::new(Expression::Literal(Value::Number(2.0))),
        };

        assert_eq!(expression.evaluate(), Value::Number(3.0));
    }

    #[test]
    fn test_evaluate_grouping() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Literal(Value::Number(7.0))),
            operator: Operator::Asterisk,
            right: Box::new(Expression::Grouping(Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Value::Number(1.0))),
                operator: Operator::Plus,
                right: Box::new(Expression::Literal(Value::Number(2.0))),
            }))),
        };

        assert_eq!(expression.evaluate(), Value::Number(21.0));
    }

    #[test]
    fn test_evaluate_unary() {
        let expression = Expression::Unary {
            operator: Operator::Minus,
            right: Box::new(Expression::Literal(Value::Number(1.0))),
        };

        assert_eq!(expression.evaluate(), Value::Number(-1.0));
    }
}
