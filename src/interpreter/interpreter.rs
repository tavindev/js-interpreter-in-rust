use crate::parser::statements::statement::Statement;

pub struct Interpreter {}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Self {
        Interpreter {}
    }
}

#[cfg(test)]
mod test {}
