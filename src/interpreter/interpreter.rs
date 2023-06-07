use std::{
    cell::{RefCell, RefMut},
    env,
    ops::Deref,
    rc::Rc,
};

use super::{
    environment::{self, Environment},
    functions::js_function::JsFunction,
};
use crate::parser::{
    expression::Expression,
    operator::Operator,
    statements::{block::BlockStatement, function::FunctionStatement, statement::Statement},
    value::Value,
};

pub struct Interpreter {
    statements: Vec<Statement>,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter { statements }
    }

    pub fn execute_block(
        &mut self,
        block: BlockStatement,
        environment: &mut Rc<RefCell<Environment>>,
    ) -> Value {
        let mut return_value = Value::Null;
        let mut environment = environment.borrow_mut();

        for statement in block.statements() {
            dbg!(statement);
            if let Some(value) = self.execute(statement, &mut environment) {
                return_value = value;
                break;
            }
            dbg!(&environment);
        }

        return return_value;
    }

    pub fn evaluate(&mut self, expr: &Expression, environment: &mut RefMut<Environment>) -> Value {
        match expr {
            Expression::Assignement { ident, value } => {
                let name = ident.value();

                if !environment.has(&name) {
                    panic!("Undefined variable: {}", name);
                }

                let value = self.evaluate(value, environment);
                environment.assign(&name, value.clone());

                return value;
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(&left, environment);
                let right = self.evaluate(&right, environment);

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
            Expression::Grouping(expression) => self.evaluate(&expression, environment),
            Expression::Literal(value) => value.clone(),
            Expression::Unary { operator, right } => {
                let right = self.evaluate(&right, environment);

                match operator {
                    Operator::Minus => Value::Number(-right.to_number()),
                    Operator::Bang => Value::Bool(!right.is_truthy()),
                    _ => unimplemented!(),
                }
            }
            Expression::Variable(ident) => {
                let name = ident.value();

                return environment.get(&name).clone();
            }
            Expression::Call { callee, arguments } => {
                let callee = self.evaluate(callee, environment);

                if let Value::Function(function) = callee {
                    let arguments = arguments
                        .into_iter()
                        .map(|argument| self.evaluate(argument, environment))
                        .collect::<Vec<Value>>();

                    if function.arity() != arguments.len() {
                        panic!(
                            "Expected {} arguments but got {}",
                            function.arity(),
                            arguments.len()
                        );
                    }

                    dbg!(function.name());

                    return function.call(self, arguments);
                } else {
                    panic!("Can only call functions and classes, got {:?}", callee);
                }
            }
        }
    }

    fn execute(
        &mut self,
        statement: &Statement,
        environment: &mut RefMut<Environment>,
    ) -> Option<Value> {
        match statement {
            Statement::Print(stmt) => {
                let value = self.evaluate(stmt, environment);
                println!("{:?}", value);
            }
            Statement::Let(stmt) => {
                let ident = stmt.ident.clone();
                let name = ident.value();

                if let Some(expression) = &stmt.expression {
                    let value = self.evaluate(&expression, environment);

                    environment.define(name, value.clone());
                } else {
                    environment.define(name, Value::Null);
                };
            }
            Statement::If(stmt) => {
                let condition = self.evaluate(&stmt.condition, environment);

                if condition.is_truthy() {
                    self.execute(&stmt.consequence, environment);
                } else if let Some(alternative) = &stmt.alternative {
                    self.execute(&alternative, environment);
                }
            }
            Statement::While(stmt) => {
                while self.evaluate(&stmt.condition, environment).is_truthy() {
                    self.execute(&stmt.body, environment);
                }
            }
            Statement::Block(stmt) => {
                for statement in stmt.statements() {
                    self.execute(statement, environment);
                }
            }
            Statement::Expression(stmt) => {
                Some(self.evaluate(stmt, environment));
            }
            Statement::Function(FunctionStatement {
                ident,
                parameters,
                body,
            }) => {
                // let function = Value::function(JsFunction::new(
                //     ident.clone(),
                //     parameters.clone(),
                //     body.clone(),
                //     environment,
                // ));

                // environment.define(ident.value(), function);
                todo!()
            }
            Statement::Return(value) => {
                return Some(self.evaluate(value, environment));
            }
        }

        None
    }

    pub fn run(&mut self, environment: &mut Rc<RefCell<Environment>>) {
        let statements = self.statements.clone();
        let mut environment = environment.borrow_mut();

        for statement in statements {
            self.execute(&statement, &mut environment);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Parser;

    struct EnvironmentHelper {
        environment: Rc<RefCell<Environment>>,
    }

    impl EnvironmentHelper {
        fn get(&self, name: &str) -> Value {
            self.environment.borrow().get(name).clone()
        }
    }

    struct RunResult {
        interpreter: Interpreter,
        environment: EnvironmentHelper,
    }

    fn run_interpreter(code: &str) -> RunResult {
        let mut environment = Rc::new(RefCell::new(Environment::new()));
        let statements = Parser::new(code).parse();

        let mut interpreter = Interpreter::new(statements);

        interpreter.run(&mut environment);

        RunResult {
            interpreter,
            environment: EnvironmentHelper { environment },
        }
    }

    #[test]
    fn variable_declaration() {
        let interpreter = run_interpreter("let x = 1; let y;");

        assert_eq!(interpreter.environment.get("x"), Value::Number(1.0));
        assert_eq!(interpreter.environment.get("y"), Value::Null);
    }

    #[test]
    fn variable_assignment() {
        let interpreter = run_interpreter("let x = 1; x = 2;");

        assert_eq!(interpreter.environment.get("x"), Value::Number(2.0));
    }

    #[test]
    #[should_panic(expected = "Undefined variable: x")]
    fn variable_assignment_with_undefined_variable() {
        run_interpreter("x = 2;");
    }

    #[test]
    fn if_statement() {
        let interpreter = run_interpreter("let x = 1; if (true) { x = 2; }");

        assert_eq!(interpreter.environment.get("x"), Value::Number(2.0));
    }

    #[test]
    fn function_return_value() {
        let interpreter = run_interpreter(
            "function foo() {
                return 1;
            }
            
            let a = foo();",
        );

        assert_eq!(interpreter.environment.get("a"), Value::Number(1.0));
    }

    #[test]
    fn closures() {
        let interpreter = run_interpreter(
            "
        function makeCounter() {
            let i = 0;
            
            function count() {
                i = i + 1;
                return i; 
            }
        
            return count;
        }
        
        let counter = makeCounter();
        let a = counter();
        let b = counter();",
        );

        assert_eq!(interpreter.environment.get("a"), Value::Number(1.0));
        assert_eq!(interpreter.environment.get("b"), Value::Number(2.0));
    }
}
