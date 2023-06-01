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

    /**
     * primary -> NUMBER | STRING | "true" | "false" | null | "(" expression ")" ;
     */
    fn primary(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Ident(ident) => return Expression::Literal(Value::Ident(ident)),
            Token::Number(int) => return Expression::Literal(Value::Number(int)),
            Token::String(string) => return Expression::Literal(Value::String(string)),
            Token::True => return Expression::Literal(Value::Bool("true".into())),
            Token::False => return Expression::Literal(Value::Bool("false".into())),
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

        let condition = self.expression();

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
                let expression = self.expression();

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
                Expression::Binary {
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
                Expression::Binary {
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
                Expression::Binary {
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
                Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Literal(Value::Number("3".into()))),
                        operator: Operator::Minus,
                        right: Box::new(Expression::Literal(Value::Number("4".into()))),
                    }),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Literal(Value::Number("1".into()))),
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
                Expression::Binary {
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
