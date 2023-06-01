use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(String),
    String(String),
    Ident(String),
    Bool(String),
    Null,
}

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
