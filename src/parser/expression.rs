use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(String),
    String(String),
    Ident(String),
    Bool(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Value),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
