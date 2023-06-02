use crate::{
    lexer::lexer::{Lexer, Token},
    parser::operator::Operator,
};

use super::{
    expression::Expression,
    ident::Ident,
    statements::{
        block::BlockStatement, r#if::IfStatement, r#let::LetStatement, statement::Statement,
    },
    value::Value,
};

pub struct Parser {
    lexer: Lexer,
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
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.lexer.is_at_end() {
            statements.push(self.statement());
            self.lexer.match_token_and_consume(Token::Semicolon);
        }

        statements
    }

    fn statement(&mut self) -> Statement {
        // repeating, yes, but expression_statement should not consume first token
        let statement = match self.lexer.peek_token() {
            Token::Let => {
                self.lexer.next_token();

                self.let_statement()
            }
            Token::If => {
                self.lexer.next_token();

                self.if_statement()
            }
            Token::LSquirly => {
                self.lexer.next_token();

                self.block_statement()
            }
            _ => return self.expression_statement(),
        };

        return statement;
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

    fn let_statement(&mut self) -> Statement {
        let ident = self.parse_ident();

        let mut expression: Option<Expression> = None;

        if self.lexer.match_token_and_consume(Token::Assign) {
            expression = Some(self.expression());
        }

        if self.lexer.next_token() != Token::Semicolon {
            panic!("Expected a semicolon");
        }

        return Statement::_let(ident, expression);
    }

    fn if_statement(&mut self) -> Statement {
        if self.lexer.next_token() != Token::Lparen {
            panic!("Expected a left parenthesis");
        }

        let condition = self.expression();

        if self.lexer.next_token() != Token::Rparen {
            panic!("Expected a right parenthesis");
        }

        if self.lexer.next_token() != Token::LSquirly {
            panic!("Expected a left brace");
        }

        let consequence = self.block_statement();

        let alternative = if self.lexer.match_token_and_consume(Token::Else) {
            if self.lexer.next_token() != Token::LSquirly {
                panic!("Expected a left brace");
            }

            let alternative = self.block_statement();

            Some(alternative)
        } else {
            None
        };

        return Statement::_if(condition, consequence, alternative);
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        return Statement::_expression(expression);
    }

    fn block_statement(&mut self) -> Statement {
        let mut statements = Vec::new();

        while self.lexer.peek_token() != Token::RSquirly && self.lexer.peek_token() != Token::Eof {
            statements.push(self.statement());
        }

        if self.lexer.next_token() != Token::RSquirly {
            panic!("Expected a right brace");
        }

        return Statement::_block(statements);
    }

    /**
     * primary -> NUMBER | STRING | "true" | "false" | null | "(" expression ")" ;
     */
    fn primary(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Ident(ident) => Expression::Literal(Value::Ident(ident)),
            Token::Number(int) => {
                Expression::Literal(Value::Number(int.parse().expect("Expected a number")))
            }
            Token::String(string) => Expression::Literal(Value::String(string)),
            Token::True => Expression::Literal(Value::Bool(true)),
            Token::False => Expression::Literal(Value::Bool(false)),
            Token::Null => Expression::Literal(Value::Null),
            Token::Lparen => {
                let expr = self.expression();

                if self.lexer.next_token() != Token::Rparen {
                    panic!("Expected a closing parenthesis");
                }

                Expression::grouping(expr)
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

                return Expression::unary(operator, right);
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

                    expr = Expression::binary(expr, operator, right);
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

                    expr = Expression::binary(expr, operator, right);
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

                    expr = Expression::binary(expr, operator, right);
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

                    expr = Expression::binary(expr, operator, right);
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
mod tests {
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

        assert_eq!(expr, Expression::literal(Value::Number(1.0)));
    }

    #[test]
    fn binary_expression() {
        let mut parser = Parser::new(s!("1 + 2;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::binary(
                Expression::literal(Value::Number(1.0)),
                Operator::Plus,
                Expression::literal(Value::Number(2.0)),
            )
        );
    }

    #[test]
    fn grouping_expression() {
        let mut parser = Parser::new(s!("(1 + 2);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::grouping(Expression::binary(
                Expression::literal(Value::Number(1.0)),
                Operator::Plus,
                Expression::literal(Value::Number(2.0)),
            ))
        );
    }

    #[test]
    fn unary_expression() {
        let mut parser = Parser::new(s!("!true;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::unary(Operator::Bang, Expression::literal(Value::Bool(true)))
        );
    }

    #[test]
    fn unary_expression_with_grouping() {
        let mut parser = Parser::new(s!("!(!true);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::unary(
                Operator::Bang,
                Expression::grouping(Expression::unary(
                    Operator::Bang,
                    Expression::literal(Value::Bool(true))
                ))
            )
        );
    }

    #[test]
    fn unary_expression_with_grouping_and_binary() {
        let mut parser = Parser::new(s!("!(!true + 1);"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::unary(
                Operator::Bang,
                Expression::grouping(Expression::binary(
                    Expression::unary(Operator::Bang, Expression::literal(Value::Bool(true))),
                    Operator::Plus,
                    Expression::literal(Value::Number(1.0)),
                ))
            )
        );
    }

    #[test]
    fn binary_expression_with_precedence() {
        let mut parser = Parser::new(s!("1 + 2 * 3;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::binary(
                Expression::literal(Value::Number(1.0)),
                Operator::Plus,
                Expression::binary(
                    Expression::literal(Value::Number(2.0)),
                    Operator::Asterisk,
                    Expression::literal(Value::Number(3.0)),
                ),
            )
        );
    }

    #[test]
    fn binary_expression_with_precedence_and_grouping() {
        let mut parser = Parser::new(s!("(1 + 2) * 3;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::binary(
                Expression::grouping(Expression::binary(
                    Expression::literal(Value::Number(1.0)),
                    Operator::Plus,
                    Expression::literal(Value::Number(2.0)),
                )),
                Operator::Asterisk,
                Expression::literal(Value::Number(3.0)),
            )
        );
    }

    /**
     * STATEMENTS
     */
    #[test]
    fn let_statement_uninitialized() {
        let mut parser = Parser::new(s!("let a;"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::Let(LetStatement {
                    ident: Ident(s!("a")),
                    expression: None,
                })
            );
        }
    }

    #[test]
    fn let_statement_initialized() {
        let mut parser = Parser::new(s!("let a = 1;"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::Let(LetStatement {
                    ident: Ident(s!("a")),
                    expression: Some(Expression::Literal(Value::Number(1.0))),
                })
            );
        }
    }

    // #[test]
    // fn return_statement() {
    //     let mut parser = Parser::new(s!("return 1;"));
    //     let stmt = parser.statement();

    //     assert_eq!(
    //         stmt,
    //         Statement::Return(ReturnStatement {
    //             expression: Some(Expression::Literal(Value::Number(1.0))),
    //         })
    //     );
    // }

    #[test]
    fn expression_statement() {
        let mut parser = Parser::new(s!("1;"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::Expression(Expression::Literal(Value::Number(1.0)))
            );
        }
    }

    #[test]
    fn block_statement() {
        let mut parser = Parser::new(s!("{ 1; }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::Block(BlockStatement(vec![Statement::Expression(
                    Expression::Literal(Value::Number(1.0))
                )],))
            );
        }
    }

    #[test]
    fn empty_block_statement() {
        let mut parser = Parser::new(s!("{ }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(stmt, Statement::Block(BlockStatement(vec![],)));
        }
    }

    #[test]
    fn if_statement() {
        let mut parser = Parser::new(s!("if (true) { 1; }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::If(IfStatement {
                    condition: Expression::Literal(Value::Bool(true)),
                    consequence: Box::new(Statement::Block(BlockStatement(vec![
                        Statement::Expression(Expression::Literal(Value::Number(1.0)))
                    ],))),
                    alternative: None,
                })
            );
        }
    }

    // #[test]
    // fn if() {
    //     let mut parser = Parser::new(s!("if (a) { } else { a = true; }"));
    //     let stmt = parser.parse();

    //     for stmt in stmt {
    //         assert_eq!(
    //             stmt,
    //             Statement::If(IfStatement {
    //                 condition: Expression::Literal(Ident(s!("a"))),
    //                 consequence: Box::new(Statement::Block(BlockStatement(vec![],))),
    //                 alternative: Some(Box::new(Statement::Block(BlockStatement(vec![
    //                     Statement::Expression(Expression::Assign {
    //                         ident: Ident(s!("a")),
    //                         expression: Box::new(Expression::Literal(Value::Bool(true))),
    //                     })
    //                 ],)))),
    //             })
    //         );
    //     }
    // }
}
