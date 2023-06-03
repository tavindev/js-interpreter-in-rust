use super::environment::Environment;
use crate::parser::{expression::Expression, statements::statement::Statement, value::Value};

pub struct Interpreter {
    statements: Vec<Statement>,
    environment: Environment,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter {
            statements,
            environment: Environment::new(),
        }
    }

    fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Let(stmt) => {
                let ident = stmt.ident.clone();
                let name = ident.value();

                if let Some(expression) = stmt.expression {
                    let value = expression.evaluate();

                    self.environment.define(name, value);
                } else {
                    self.environment.define(name, Value::Null);
                }
            }
            Statement::If(stmt) => {
                let condition = stmt.condition.evaluate();

                if condition.is_truthy() {
                    self.execute(*stmt.consequence);
                } else if let Some(alternative) = stmt.alternative {
                    self.execute(*alternative);
                }
            }
            Statement::While(stmt) => {
                let condition = stmt.condition.evaluate();

                while condition.is_truthy() {
                    self.execute(*stmt.body.clone());
                }
            }
            Statement::Block(stmt) => {
                for statement in stmt.statements() {
                    self.execute(statement.clone());
                }
            }
            Statement::Expression(stmt) => match stmt {
                Expression::Assignement { ident, value } => {
                    let name = ident.clone().value();

                    if !self.environment.has(&name) {
                        panic!("Undefined variable: {}", name);
                    }

                    let value = value.evaluate();
                    self.environment.define(name, value);
                }
                _ => {}
            },
        }
    }

    pub fn run(&mut self) {
        for statement in self.statements.clone() {
            self.execute(statement);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{expression::Expression, ident::Ident};

    use super::*;

    #[test]
    fn variable_declaration() {
        let statements = vec![
            Statement::_let(
                Ident::new("x"),
                Some(Expression::literal(Value::Number(1.0))),
            ),
            Statement::_let(Ident::new("y"), None),
        ];

        let mut interpreter = Interpreter::new(statements);

        interpreter.run();

        assert_eq!(interpreter.environment.get("x"), &Value::Number(1.0));

        assert_eq!(interpreter.environment.get("y"), &Value::Null);
    }

    #[test]
    fn variable_assignment() {
        let statements = vec![
            Statement::_let(
                Ident::new("x"),
                Some(Expression::literal(Value::Number(1.0))),
            ),
            Statement::_expression(Expression::assignement(
                Ident::new("x"),
                Expression::literal(Value::Number(2.0)),
            )),
        ];

        let mut interpreter = Interpreter::new(statements);

        interpreter.run();

        assert_eq!(interpreter.environment.get("x"), &Value::Number(2.0));
    }

    #[test]
    #[should_panic(expected = "Undefined variable: x")]
    fn variable_assignment_with_undefined_variable() {
        let statements = vec![Statement::_expression(Expression::assignement(
            Ident::new("x"),
            Expression::literal(Value::Number(2.0)),
        ))];

        let mut interpreter = Interpreter::new(statements);

        interpreter.run();
    }

    #[test]
    fn if_statement() {
        let statements = vec![
            Statement::_let(
                Ident::new("x"),
                Some(Expression::literal(Value::Number(1.0))),
            ),
            Statement::_if(
                Expression::Literal(Value::Bool(true)),
                Statement::_block(vec![Statement::_expression(Expression::assignement(
                    Ident::new("x"),
                    Expression::literal(Value::Number(2.0)),
                ))]),
                None,
            ),
        ];

        let mut interpreter = Interpreter::new(statements);

        interpreter.run();

        assert_eq!(interpreter.environment.get("x"), &Value::Number(2.0));
    }
}
