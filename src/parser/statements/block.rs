use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct BlockStatement(pub Vec<Statement>);
