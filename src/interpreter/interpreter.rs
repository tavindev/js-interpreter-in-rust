use std::collections::HashMap;

use crate::parser::{
    expression::Expression,
    ident::Ident,
    statements::{r#let::LetStatement, statement::Statement},
};

struct Environment {
    store: HashMap<Ident, Expression>,
}

pub struct Interpreter {
    statements: Vec<Statement>,
    env: Environment,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter {
            statements,
            env: Environment {
                store: HashMap::new(),
            },
        }
    }

    pub fn run(&mut self) {
        for statement in self.statements.iter() {
            if let Statement::Let(LetStatement { name, expression }) = statement {
                self.env.store.insert(name.clone(), expression.clone());
            }
        }

        println!("{:?}", self.env.store);
    }
}

#[cfg(test)]
mod test {}
