use super::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement(pub Vec<Statement>);
