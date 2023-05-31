use crate::{
    lexer::lexer::{Lexer, Token},
    parser::operator::Operator,
};

use super::{
    expression::Expression,
    ident::Ident,
    statements::{r#let::LetStatement, statement::Statement},
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
        while let Ok(token) = self.lexer.next_token() {
            let mut statement = Option::None;

            match token {
                Token::Let => statement = Some(self.parse_let_statement()),
                Token::If => statement = Some(self.parse_if_statement()),
                Token::Eof => break,
                _ => {}
            }

            if let Some(statement) = statement {
                self.statements.push(statement);
            }
        }

        return self.statements.clone();
    }

    fn parse_ident(&mut self) -> Ident {
        if let Ok(token) = self.lexer.next_token() {
            match token {
                Token::Ident(ident) => return Ident(ident),
                _ => panic!("Expected an identifier"),
            }
        } else {
            panic!("Error");
        }
    }

    fn parse_operator_expression(&mut self) -> Expression {
        let left = self.parse_literal_expression();

        let operator = match self.lexer.next_token() {
            Ok(token) => match token {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => panic!("Expected an operator"),
            },
            Err(_) => panic!("Error"),
        };

        let right = self.parse_expression();

        return Expression::Operator {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    fn parse_literal_expression(&mut self) -> Expression {
        let token = self.lexer.curr_token();

        match token {
            Token::Int(_) | Token::String(_) | Token::Ident(_) => Expression::Literal(token),
            t => panic!("Expected an int or string, got {:?}", t),
        }
    }

    fn parse_expression(&mut self) -> Expression {
        if let Ok(token) = self.lexer.next_token() {
            match token {
                Token::Int(_) | Token::String(_) | Token::Ident(_) => {
                    if let Ok(next_token) = self.lexer.peek_token() {
                        return match next_token {
                            Token::Plus | Token::Minus => self.parse_operator_expression(),
                            Token::Semicolon | Token::Eof | Token::Newline => {
                                self.parse_literal_expression()
                            }
                            _ => panic!("Expected an operator or value, got {:?}", next_token),
                        };
                    } else {
                        panic!("Error");
                    }
                }
                _ => panic!("Expected an operator or value, got {:?}", token),
            };
        } else {
            panic!("Error");
        }
    }

    fn parse_if_statement(&mut self) -> Statement {
        todo!("Implement if statement")
    }

    fn parse_let_statement(&mut self) -> Statement {
        let name = self.parse_ident();

        if let Ok(token) = self.lexer.next_token() {
            match token {
                Token::Assign => {
                    let expression = self.parse_expression();
                    return Statement::Let(LetStatement { name, expression });
                }
                // We'll later add other types of assignment operators
                _ => panic!("Expected an equal sign"),
            }
        } else {
            panic!("Error");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::operator::Operator;

    use super::*;

    #[test]
    fn parse_let_statement_literal() {
        let input = "let x = 3;";
        let parser = Parser::new(input.into());

        assert_eq!(parser.statements.len(), 1);

        if let Statement::Let(statement) = &parser.statements[0] {
            assert_eq!(statement.name.0, "x");
            assert_eq!(
                statement.expression,
                Expression::Literal(Token::Int("3".into()))
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
                    left: Box::new(Expression::Literal(Token::Int("3".into()))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Token::Int("4".into()))),
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
                    left: Box::new(Expression::Literal(Token::Int("3".into()))),
                    operator: Operator::Minus,
                    right: Box::new(Expression::Operator {
                        right: Box::new(Expression::Literal(Token::Int("1".into()))),
                        operator: Operator::Plus,
                        left: Box::new(Expression::Literal(Token::Int("4".into()))),
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
                Expression::Literal(Token::Ident("y".into()))
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
                    left: Box::new(Expression::Literal(Token::Ident("y".into()))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Token::Ident("z".into()))),
                }
            );
        }
    }
}
