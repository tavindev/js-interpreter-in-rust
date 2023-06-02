use super::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement(Vec<Statement>);

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> BlockStatement {
        BlockStatement(statements)
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.0
    }
}
