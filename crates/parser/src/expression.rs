use crate::{ident::Ident, operator::Operator, value::Value};

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
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    pub fn grouping(expression: Expression) -> Expression {
        Expression::Grouping(Box::new(expression))
    }

    pub fn literal(value: Value) -> Expression {
        Expression::Literal(value)
    }

    pub fn call(callee: Expression, arguments: Vec<Expression>) -> Expression {
        Expression::Call {
            callee: Box::new(callee),
            arguments,
        }
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

    pub fn variable<S: Into<String>>(ident: S) -> Expression {
        Expression::Variable(Ident::new(ident.into()))
    }
}

#[cfg(test)]
mod tests {}
