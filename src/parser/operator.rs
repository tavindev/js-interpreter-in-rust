#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    And,
    Or,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
