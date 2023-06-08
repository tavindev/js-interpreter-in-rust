use std::rc::Rc;

use crate::{functions::js_function::JsFunction, value::Value};

use parser::value::ParserValue;

use super::environment::Environment;
use parser::{
    expression::Expression,
    operator::Operator,
    statements::{block::BlockStatement, function::FunctionStatement, statement::Statement},
};

pub struct Interpreter {
    statements: Vec<Statement>,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter { statements }
    }

    pub fn execute_block(&mut self, block: BlockStatement, environment: &Rc<Environment>) -> Value {
        let mut return_value = Value::Null;

        for statement in block.statements() {
            if let Some(value) = self.execute(statement, &environment) {
                return_value = value;
                break;
            }
        }

        return return_value;
    }

    pub fn evaluate(&mut self, expr: &Expression, environment: &Rc<Environment>) -> Value {
        match expr {
            Expression::Assignement { ident, value } => {
                let name = ident.value();

                if !environment.has(&name) {
                    panic!("Undefined variable: {}", name);
                }

                let mut value = self.evaluate(value, environment);

                if let Value::Function(function) = &mut value {
                    function.set_name(name.clone())
                }

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
            Expression::Literal(value) => match value {
                ParserValue::String(string) => Value::String(string.clone()),
                ParserValue::Number(number) => Value::Number(
                    number
                        .parse::<f64>()
                        .expect("Could not parse number from string"),
                ),
                ParserValue::Bool(boolean) => Value::Bool(*boolean),
                ParserValue::Null => Value::Null,
                ParserValue::Function {
                    ident,
                    params,
                    body,
                } => Value::Function(JsFunction::new(
                    ident.clone(),
                    params.clone(),
                    body.clone(),
                    Rc::clone(&environment),
                )),
            },
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

                    return function.call(self, arguments);
                } else {
                    panic!("Can only call functions and classes, got {:?}", callee);
                }
            }
        }
    }

    fn execute(&mut self, statement: &Statement, environment: &Rc<Environment>) -> Option<Value> {
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
                let function = Value::function(JsFunction::new(
                    Some(ident.clone()),
                    parameters.clone(),
                    body.clone(),
                    Rc::clone(&environment),
                ));

                environment.define(ident.value(), function);
            }
            Statement::Return(value) => {
                return Some(self.evaluate(value, environment));
            }
        }

        None
    }

    pub fn run(&mut self, environment: &Rc<Environment>) {
        let statements = self.statements.clone();

        for statement in statements {
            self.execute(&statement, &environment);
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::parser::Parser;

    use super::*;

    struct EnvironmentHelper {
        environment: Rc<Environment>,
    }

    impl EnvironmentHelper {
        fn get(&self, name: &str) -> Value {
            self.environment.get(name).clone()
        }
    }

    struct RunResult {
        environment: EnvironmentHelper,
    }

    fn run_interpreter(code: &str) -> RunResult {
        let environment = Rc::new(Environment::new());
        let statements = Parser::new(code).parse();

        let mut interpreter = Interpreter::new(statements);

        interpreter.run(&environment);

        RunResult {
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

    #[test]
    fn let_functions() {
        let interpreter = run_interpreter(
            "
        let foo = function() {
            return 1;
        };
        
        let a = foo();",
        );

        assert_eq!(interpreter.environment.get("a"), Value::Number(1.0));
    }
}
