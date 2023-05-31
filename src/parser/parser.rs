use crate::{
    lexer::lexer::{Lexer, Token},
    parser::operator::Operator,
};

use super::{
    expression::{Expression, Value},
    ident::Ident,
    statements::{
        block::BlockStatement, r#if::IfStatement, r#let::LetStatement, statement::Statement,
    },
};

pub struct Parser {
    lexer: Lexer,
    statements: Vec<Statement>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut parser = Parser {
            lexer: Lexer::new(input),
            statements: Vec::new(),
        };

        parser.parse();

        return parser;
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        loop {
            let statement = match self.lexer.next_token() {
                Token::Let => Some(self.parse_let_statement()),
                Token::If => Some(self.parse_if_statement()),
                Token::Eof => break,
                _ => None,
            };

            if let Some(statement) = statement {
                self.statements.push(statement);
            }
        }

        return self.statements.clone();
    }

    fn parse_ident(&mut self) -> Ident {
        match self.lexer.next_token() {
            Token::Ident(ident) => return Ident(ident),
            _ => panic!("Expected an identifier"),
        }
    }

    fn parse_token_to_value(&mut self, token: Token) -> Value {
        match token {
            Token::Number(int) => Value::Number(int),
            Token::String(string) => Value::String(string),
            Token::Ident(ident) => Value::Ident(ident),
            Token::True => Value::Bool("true".into()),
            Token::False => Value::Bool("false".into()),
            t => panic!("Cannot parse {:?} to Value", t),
        }
    }

    fn parse_token_to_operator(&mut self, token: Token) -> Operator {
        match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Asterisk => Operator::Asterisk,
            Token::ForwardSlash => Operator::Slash,
            Token::Equal => Operator::Equal,
            Token::NotEqual => Operator::NotEqual,
            Token::And => Operator::And,
            Token::Or => Operator::Or,
            Token::LessThan => Operator::LessThan,
            Token::LessThanOrEqual => Operator::LessThanOrEqual,
            Token::GreaterThan => Operator::GreaterThan,
            Token::GreaterThanOrEqual => Operator::GreaterThanOrEqual,
            token => panic!("Expected an operator, got {:?}", token),
        }
    }

    fn parse_operator_expression(&mut self) -> Expression {
        let left = self.parse_literal_expression();
        let token = self.lexer.next_token();
        let operator = self.parse_token_to_operator(token);
        let right = self.parse_expression();

        return Expression::Operator {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    fn parse_literal_expression(&mut self) -> Expression {
        let token = self.lexer.curr_token();

        return Expression::Literal(self.parse_token_to_value(token));
    }

    fn parse_expression(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Number(_) | Token::String(_) | Token::Ident(_) | Token::True | Token::False => {
                match self.lexer.peek_token() {
                    Token::Plus
                    | Token::Minus
                    | Token::Asterisk
                    | Token::ForwardSlash
                    | Token::Equal
                    | Token::NotEqual
                    | Token::And
                    | Token::Or
                    | Token::LessThan
                    | Token::LessThanOrEqual
                    | Token::GreaterThan
                    | Token::GreaterThanOrEqual => self.parse_operator_expression(),
                    Token::Semicolon
                    | Token::Eof
                    | Token::Newline
                    | Token::RSquirly
                    | Token::Rparen => self.parse_literal_expression(),
                    next_token => panic!("Expected an operator or value, got {:?}", next_token),
                }
            }
            token => panic!("Expected an expression or value, got {:?}", token),
        }
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        match self.lexer.next_token() {
            Token::LSquirly => {
                let mut statements = Vec::new();

                loop {
                    match self.lexer.next_token() {
                        Token::Let => statements.push(self.parse_let_statement()),
                        Token::If => statements.push(self.parse_if_statement()),
                        Token::RSquirly => break,
                        _ => {}
                    }
                }

                BlockStatement(statements)
            }
            _ => {
                panic!("Expected a left brace");
            }
        }
    }

    fn parse_if_statement(&mut self) -> Statement {
        let token = self.lexer.next_token();

        if token != Token::Lparen {
            panic!("Expected a left parenthesis");
        };

        let condition = self.parse_expression();

        let token = self.lexer.next_token();

        if token != Token::Rparen {
            panic!("Expected a right parenthesis");
        }

        let consequence = self.parse_block_statement();

        let alternative = match self.lexer.next_token() {
            Token::Else => Some(self.parse_block_statement()),
            _ => None,
        };

        Statement::If(IfStatement {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_let_statement(&mut self) -> Statement {
        let name = self.parse_ident();

        match self.lexer.next_token() {
            Token::Assign => {
                let expression = self.parse_expression();

                Statement::Let(LetStatement { name, expression })
            }
            // We'll later add other types of assignment operators
            _ => panic!("Expected an equal sign"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{expression::Value, operator::Operator};

    use super::*;

    /**
     * EXPRESSIONS
     */
    #[test]
    fn parse_expression_literal() {
        let input = "let x = 3 == y;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Operator {
                    left: Box::new(Expression::Literal(Value::Number("3".into()))),
                    operator: Operator::Equal,
                    right: Box::new(Expression::Literal(Value::Ident("y".into()))),
                }
            );
        }
    }

    #[test]
    fn parse_expression_literal_bool() {
        let input = "let x = true == false;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Operator {
                    left: Box::new(Expression::Literal(Value::Bool("true".into()))),
                    operator: Operator::Equal,
                    right: Box::new(Expression::Literal(Value::Bool("false".into()))),
                }
            );
        }
    }

    /**
     * LET STATEMENT
     */
    #[test]
    fn parse_let_statement_literal() {
        let input = "let x = 3;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Literal(Value::Number("3".into()))
            );
        }
    }

    #[test]
    fn parse_let_statement_expression() {
        let input = "let x = 3 + 4;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Operator {
                    left: Box::new(Expression::Literal(Value::Number("3".into()))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Value::Number("4".into()))),
                }
            );
        }
    }

    #[test]
    fn parse_let_statement_compose_expression() {
        let input = "let x = 3 - 4 + 1;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Operator {
                    left: Box::new(Expression::Literal(Value::Number("3".into()))),
                    operator: Operator::Minus,
                    right: Box::new(Expression::Operator {
                        right: Box::new(Expression::Literal(Value::Number("1".into()))),
                        operator: Operator::Plus,
                        left: Box::new(Expression::Literal(Value::Number("4".into()))),
                    }),
                }
            );
        }
    }

    #[test]
    fn parse_let_statement_ident_expression() {
        let input = "let x = y;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Literal(Value::Ident("y".into()))
            );
        }
    }

    #[test]
    fn parse_let_statement_ident_sum() {
        let input = "let x = y + z;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Operator {
                    left: Box::new(Expression::Literal(Value::Ident("y".into()))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Value::Ident("z".into()))),
                }
            );
        }
    }

    /**
     * BLOCK STATEMENT
     */
    #[test]
    fn parse_empty_block_statement() {
        let input = "{}";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 0);
    }

    #[test]
    fn parse_block_statement_literal() {
        let input = "{ let x = 3; }";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Block(statement) = &parser.statements[0] {
            assert_eq!(statement.0.len(), 1);

            if let Statement::Let(let_statement) = &statement.0[0] {
                assert_eq!(let_statement.name.0, "x");
                assert_eq!(
                    let_statement.expression,
                    Expression::Literal(Value::Number("3".into()))
                );
            }
        }
    }

    /**
     * IF STATEMENT
     */
    #[test]
    fn parse_if_statement_no_consequence() {
        let input = "if (3) {}";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::If(statement) = &parser.statements[0] {
            assert_eq!(
                statement.condition,
                Expression::Literal(Value::Number("3".into()))
            );
            assert_eq!(statement.consequence.0.len(), 0);
        }
    }

    #[test]
    fn parse_if_statement_no_alternative() {
        let input = "if (3) { let x = 3; }";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::If(statement) = &parser.statements[0] {
            assert_eq!(
                statement.condition,
                Expression::Literal(Value::Number("3".into()))
            );
            assert_eq!(statement.consequence.0.len(), 1);

            if let Statement::Let(let_statement) = &statement.consequence.0[0] {
                assert_eq!(let_statement.name.0, "x");
                assert_eq!(
                    let_statement.expression,
                    Expression::Literal(Value::Number("3".into()))
                );
            }

            assert_eq!(statement.alternative.is_none(), true);
        }
    }

    #[test]
    fn parse_if_statement_alternative() {
        let input = "if (3) { let x = 3; } else { let y = 4; }";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::If(statement) = &parser.statements[0] {
            assert_eq!(
                statement.condition,
                Expression::Literal(Value::Number("3".into()))
            );
            assert_eq!(statement.consequence.0.len(), 1);

            if let Statement::Let(let_statement) = &statement.consequence.0[0] {
                assert_eq!(let_statement.name.0, "x");
                assert_eq!(
                    let_statement.expression,
                    Expression::Literal(Value::Number("3".into()))
                );
            }

            assert_eq!(statement.alternative.is_some(), true);

            if let Some(alternative) = &statement.alternative {
                assert_eq!(alternative.0.len(), 1);

                if let Statement::Let(let_statement) = &alternative.0[0] {
                    assert_eq!(let_statement.name.0, "y");
                    assert_eq!(
                        let_statement.expression,
                        Expression::Literal(Value::Number("4".into()))
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn panic_if_statement_no_condition() {
        let input = "if () {}";
        Parser::new(input.into());
    }
}
