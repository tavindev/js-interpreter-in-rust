use super::environment::Environment;
use crate::parser::{
    expression::Expression, operator::Operator, statements::statement::Statement, value::Value,
};

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

    pub fn evaluate(&self, expr: Expression) -> Value {
        match expr {
            Expression::Assignement { ident: _, value } => self.evaluate(*value),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left);
                let right = self.evaluate(*right);

                match operator {
                    Operator::Plus => left.sum(&right),
                    Operator::Minus => left.sub(&right),
                    Operator::Asterisk => left.mult(&right),
                    Operator::Slash => left.div(&right),
                    Operator::GreaterThan => left.gt(&right),
                    Operator::GreaterThanOrEqual => left.gte(&right),
                    Operator::LessThan => left.lt(&right),
                    Operator::LessThanOrEqual => left.lte(&right),
                    Operator::Equal => left.eq(&right),
                    Operator::NotEqual => left.neq(&right),
                    Operator::And => left.and(&right),
                    Operator::Or => left.or(&right),
                    _ => unimplemented!(),
                }
            }
            Expression::Grouping(expression) => self.evaluate(*expression),
            Expression::Literal(value) => value.clone(),
            Expression::Unary { operator, right } => {
                let right = self.evaluate(*right);

                match operator {
                    Operator::Minus => Value::Number(-right.to_number()),
                    Operator::Bang => Value::Bool(!right.is_truthy()),
                    _ => unimplemented!(),
                }
            }
            Expression::Variable(ident) => {
                let ident_value = ident.value();
                let name = ident_value.as_str();

                return self.environment.get(name).clone();
            }
        }
    }

    fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Print(stmt) => {
                let value = self.evaluate(stmt);
                println!("{:?}", value);
            }
            Statement::Let(stmt) => {
                let ident = stmt.ident.clone();
                let name = ident.value();

                if let Some(expression) = stmt.expression {
                    let value = self.evaluate(expression);

                    self.environment.define(name, value);
                } else {
                    self.environment.define(name, Value::Null);
                }
            }
            Statement::If(stmt) => {
                let condition = self.evaluate(stmt.condition);

                if condition.is_truthy() {
                    self.execute(*stmt.consequence);
                } else if let Some(alternative) = stmt.alternative {
                    self.execute(*alternative);
                }
            }
            Statement::While(stmt) => {
                while self.evaluate(stmt.condition.clone()).is_truthy() {
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

                    let value = self.evaluate(*value);
                    self.environment.assign(name.as_str(), value);
                }
                _ => unimplemented!(),
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
