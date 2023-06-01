use crate::{
    lexer::lexer::{Lexer, Token},
    parser::operator::Operator,
};

use super::{expression::Expression, ident::Ident, value::Value};

pub struct Parser {
    lexer: Lexer,
    expressions: Vec<Expression>,
}

/**
 * Expression grammar
 * expression -> equality ;
 * equality -> comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison -> term ( ( ">" | ">=" | "<" | ">" ) term )* ;
 * term -> factor ( ( "-" | "+" ) factor )* ;
 * factor -> unary ( ( "/" | "*" ) unary )* ;
 * unary -> ( "!" | "-" ) unary | primary ;
 * primary -> NUMBER | STRING | "true" | "false" | null | "(" expression ")" ;
 */
impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            expressions: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        loop {
            match self.lexer.peek_token() {
                Token::Eof => break,
                _ => {
                    let expr = self.expression();

                    self.expressions.push(expr);
                }
            }
        }

        return self.expressions.clone();
    }

    fn parse_ident(&mut self) -> Ident {
        match self.lexer.next_token() {
            Token::Ident(ident) => return Ident(ident),
            _ => panic!("Expected an identifier"),
        }
    }

    fn parse_token_to_operator(&mut self, token: Token) -> Operator {
        match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Asterisk => Operator::Asterisk,
            Token::ForwardSlash => Operator::Slash,
            Token::Bang => Operator::Bang,
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

    /**
     * primary -> NUMBER | STRING | "true" | "false" | null | "(" expression ")" ;
     */
    fn primary(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Ident(ident) => return Expression::Literal(Value::Ident(ident)),
            Token::Number(int) => {
                return Expression::Literal(Value::Number(int.parse().expect("Expected a number")))
            }
            Token::String(string) => return Expression::Literal(Value::String(string)),
            Token::True => return Expression::Literal(Value::Bool(true)),
            Token::False => return Expression::Literal(Value::Bool(false)),
            Token::Null => return Expression::Literal(Value::Null),
            Token::Lparen => {
                let expr = self.expression();
                if self.lexer.next_token() != Token::Rparen {
                    panic!("Expected a closing parenthesis");
                }

                return Expression::Grouping(Box::new(expr));
            }
            token => panic!("Expected a primary expression, got {:?}", token),
        }
    }

    /**
     * unary -> ( "!" | "-" ) unary | primary ;
     */
    fn unary(&mut self) -> Expression {
        match self.lexer.peek_token() {
            Token::Bang | Token::Minus => {
                let token = self.lexer.next_token();
                let operator = self.parse_token_to_operator(token);
                let right = self.unary();

                return Expression::Unary {
                    operator,
                    right: Box::new(right),
                };
            }
            _ => return self.primary(),
        }
    }

    /**
     * factor -> unary ( ( "/" | "*" ) unary )* ;
     */
    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        loop {
            match self.lexer.peek_token() {
                Token::Asterisk | Token::ForwardSlash => {
                    let token = self.lexer.next_token();
                    let operator = self.parse_token_to_operator(token);
                    let right = self.unary();

                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        return expr;
    }

    /**
     * term -> factor ( ( "-" | "+" ) factor )* ;
     */
    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        loop {
            match self.lexer.peek_token() {
                Token::Plus | Token::Minus => {
                    let token = self.lexer.next_token();
                    let operator = self.parse_token_to_operator(token);
                    let right = self.factor();

                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        return expr;
    }

    /**
     * comparison -> term ( ( ">" | ">=" | "<" | ">" ) term )* ;
     */
    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        loop {
            match self.lexer.peek_token() {
                Token::GreaterThan
                | Token::GreaterThanOrEqual
                | Token::LessThan
                | Token::LessThanOrEqual => {
                    let token = self.lexer.next_token();
                    let operator = self.parse_token_to_operator(token);
                    let right = self.term();

                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        return expr;
    }

    /**
     * equality -> comparison ( ( "!=" | "==" ) comparison )* ;
     */
    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        loop {
            match self.lexer.peek_token() {
                Token::Equal | Token::NotEqual => {
                    let token = self.lexer.next_token();
                    let operator = self.parse_token_to_operator(token);
                    let right = self.comparison();

                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        return expr;
    }

    /**
     * expression -> equality ;
     */
    fn expression(&mut self) -> Expression {
        return self.equality();
    }
}

#[cfg(test)]
mod test {
    macro_rules! s {
        ($s:expr) => {
            $s.to_string()
        };
    }

    use super::*;
    use crate::parser::expression::Expression;

    #[test]
    fn literal_expression() {
        let mut parser = Parser::new(s!("1;"));
        let expr = parser.expression();

        assert_eq!(expr, Expression::Literal(Value::Number(1.0)));
    }

    #[test]
    fn binary_expression() {
        let mut parser = Parser::new(s!("1 + 2;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Literal(Value::Number(1.0))),
                operator: Operator::Plus,
                right: Box::new(Expression::Literal(Value::Number(2.0))),
            }
        );
    }

    #[test]
    fn grouping_expression() {
        let mut parser = Parser::new(s!("(1 + 2);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Grouping(Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Value::Number(1.0))),
                operator: Operator::Plus,
                right: Box::new(Expression::Literal(Value::Number(2.0))),
            }))
        );
    }

    #[test]
    fn unary_expression() {
        let mut parser = Parser::new(s!("!true;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Unary {
                operator: Operator::Bang,
                right: Box::new(Expression::Literal(Value::Bool(true))),
            }
        );
    }

    #[test]
    fn unary_expression_with_grouping() {
        let mut parser = Parser::new(s!("!(!true);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Unary {
                operator: Operator::Bang,
                right: Box::new(Expression::Grouping(Box::new(Expression::Unary {
                    operator: Operator::Bang,
                    right: Box::new(Expression::Literal(Value::Bool(true))),
                }))),
            }
        );
    }

    #[test]
    fn unary_expression_with_grouping_and_binary() {
        let mut parser = Parser::new(s!("!(!true + 1);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Unary {
                operator: Operator::Bang,
                right: Box::new(Expression::Grouping(Box::new(Expression::Binary {
                    left: Box::new(Expression::Unary {
                        operator: Operator::Bang,
                        right: Box::new(Expression::Literal(Value::Bool(true))),
                    }),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Value::Number(1.0))),
                }))),
            }
        );
    }

    #[test]
    fn binary_expression_with_precedence() {
        let mut parser = Parser::new(s!("1 + 2 * 3;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Literal(Value::Number(1.0))),
                operator: Operator::Plus,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Literal(Value::Number(2.0))),
                    operator: Operator::Asterisk,
                    right: Box::new(Expression::Literal(Value::Number(3.0))),
                }),
            }
        );
    }

    #[test]
    fn binary_expression_with_precedence_and_grouping() {
        let mut parser = Parser::new(s!("(1 + 2) * 3;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Grouping(Box::new(Expression::Binary {
                    left: Box::new(Expression::Literal(Value::Number(1.0))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Value::Number(2.0))),
                }))),
                operator: Operator::Asterisk,
                right: Box::new(Expression::Literal(Value::Number(3.0))),
            }
        );
    }
}
