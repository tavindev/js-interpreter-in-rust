use lexer::{lexer::Lexer, token::Token};

use crate::{
    expression::Expression, ident::Ident, operator::Operator, statements::statement::Statement,
    value::ParserValue,
};

enum FunctionType {
    Function,
    // Method,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new<S: Into<String>>(input: S) -> Parser {
        Parser {
            lexer: Lexer::new(input.into()),
        }
    }

    /**
     * parse -> declaration* EOF ;
     */
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.lexer.is_at_end() {
            statements.push(self.declaration());
            self.lexer.match_token_and_consume(Token::Semicolon);
        }

        return statements;
    }

    /**
     * varDecl -> "let" IDENTIFIER ( "=" expression )? ";" ;
     */
    fn var_decl(&mut self) -> Statement {
        let ident = self.parse_ident();
        let mut expr = None;

        if self.lexer.match_token_and_consume(Token::Assign) {
            expr = Some(self.expression());
        }

        self.lexer.match_token_and_consume(Token::Semicolon);

        return Statement::_let(ident, expr);
    }

    /**
     * function -> "(" parameters? ")" block ;
     */
    fn function(&mut self) -> Expression {
        self.expect(Token::Lparen, "Expected a left parenthesis");

        let mut params = Vec::new();

        if self.lexer.peek_token() != Token::Rparen {
            loop {
                if params.len() >= 255 {
                    panic!("Cannot have more than 255 parameters");
                }

                params.push(self.parse_ident());

                if !self.lexer.match_token_and_consume(Token::Comma) {
                    break;
                }
            }
        }

        self.expect(Token::Rparen, "Expected a right parenthesis");
        self.expect(Token::LSquirly, "Expected a left brace");

        let body = if let Statement::Block(block) = self.block_statement() {
            block
        } else {
            panic!("Expected a block statement");
        };

        return Expression::literal(ParserValue::function(None, params, body));
    }

    /**
     * functionDecl -> "function" IDENTIFIER function ;
     * parameters -> IDENTIFIER ( "," IDENTIFIER )* ;
     */
    fn function_decl(&mut self, _fn_type: FunctionType) -> Statement {
        let ident = self.parse_ident();

        if let Expression::Literal(ParserValue::Function {
            ident: _,
            params,
            body,
        }) = self.function()
        {
            return Statement::function(ident, params, body);
        }

        panic!("Expected a block statement");
    }

    /**
     * declaration -> functionDecl | varDecl | statement ;
     */
    fn declaration(&mut self) -> Statement {
        if self.lexer.match_token_and_consume(Token::Function) {
            return self.function_decl(FunctionType::Function);
        }

        if self.lexer.match_token_and_consume(Token::Let) {
            return self.var_decl();
        }

        return self.statement();
    }

    /**
     * block -> "{" declaration* "}" ;
     */
    fn block_statement(&mut self) -> Statement {
        let mut statements = Vec::new();

        while self.lexer.peek_token() != Token::RSquirly && self.lexer.peek_token() != Token::Eof {
            statements.push(self.declaration());
            self.lexer.match_token_and_consume(Token::Semicolon);
        }

        self.expect(Token::RSquirly, "Expected a right brace");

        return Statement::_block(statements);
    }

    /**
     * if -> "if" "(" expression ")" statement ( "else" statement )? ;
     */
    fn if_statement(&mut self) -> Statement {
        self.expect(Token::Lparen, "Expected a left parenthesis");

        let condition = self.expression();

        self.expect(Token::Rparen, "Expected a right parenthesis");

        let consequence = self.statement();

        let alternative = if self.lexer.match_token_and_consume(Token::Else) {
            Some(self.statement())
        } else {
            None
        };

        return Statement::_if(condition, consequence, alternative);
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        return Statement::_expression(expression);
    }

    /**
     * while -> "while" "(" expression ")" statement ;
     */
    fn while_statement(&mut self) -> Statement {
        self.expect(Token::Lparen, "Expected a left parenthesis");

        let condition = self.expression();

        self.expect(Token::Rparen, "Expected a right parenthesis");

        let body = self.statement();

        return Statement::_while(condition, body);
    }

    /**
     * for -> "for" "(" ( varDecl | expression | ";" ) expression? ";" expression? ")" statement ;
     */
    pub fn for_statement(&mut self) -> Statement {
        self.expect(Token::Lparen, "Expected a left parenthesis");

        let initializer = match self.lexer.next_token() {
            Token::Let => Some(self.var_decl()),
            Token::Semicolon => None,
            _ => Some(self.expression_statement()),
        };

        let condition = if self.lexer.peek_token() != Token::Semicolon {
            self.expression()
        } else {
            Expression::Literal(ParserValue::Bool(true))
        };

        self.expect(Token::Semicolon, "Expected a semicolon");

        let increment = if self.lexer.peek_token() != Token::Rparen {
            Some(self.expression())
        } else {
            None
        };

        self.expect(Token::Rparen, "Expected a right parenthesis");

        let mut body = self.statement();

        if let Some(increment) = increment {
            body = Statement::_block(vec![body, Statement::_expression(increment)]);
        }

        body = Statement::_while(condition, body);

        if let Some(initializer) = initializer {
            body = Statement::_block(vec![initializer, body]);
        }

        return body;
    }

    /**
     * print -> "print" expression ";" ;
     */
    fn print_statement(&mut self) -> Statement {
        let expression = self.expression();

        self.lexer.match_token_and_consume(Token::Semicolon);

        return Statement::print(expression);
    }

    /**
     * return -> "return" expression? ";" ;
     */
    fn return_statement(&mut self) -> Statement {
        let value;

        if self.lexer.peek_token() != Token::Semicolon {
            value = self.expression();
        } else {
            value = Expression::Literal(ParserValue::Null);
        }

        return Statement::_return(value);
    }

    /**
     * statement -> expr | if | print | for | while | return | block ;
     */
    fn statement(&mut self) -> Statement {
        if self.lexer.match_token_and_consume(Token::If) {
            return self.if_statement();
        }

        if self.lexer.match_token_and_consume(Token::LSquirly) {
            return self.block_statement();
        }

        if self.lexer.match_token_and_consume(Token::While) {
            return self.while_statement();
        }

        if self.lexer.match_token_and_consume(Token::For) {
            return self.for_statement();
        }

        if self.lexer.match_token_and_consume(Token::Print) {
            return self.print_statement();
        }

        if self.lexer.match_token_and_consume(Token::Return) {
            return self.return_statement();
        }

        return self.expression_statement();
    }

    /**
     * primary -> NUMBER | STRING | "true" | "false" | null | "(" expression ")" | IDENTIFIER ;
     */
    fn primary(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Ident(ident) => Expression::variable(ident),
            Token::Number(int) => Expression::Literal(ParserValue::number(int)),
            Token::String(string) => Expression::Literal(ParserValue::String(string.to_string())),
            Token::True => Expression::Literal(ParserValue::Bool(true)),
            Token::False => Expression::Literal(ParserValue::Bool(false)),
            Token::Null => Expression::Literal(ParserValue::Null),
            Token::Lparen => {
                let expr = self.expression();

                self.expect(Token::Rparen, "Expected a closing parenthesis");

                Expression::grouping(expr)
            }
            token => panic!("Expected a primary expression, got {:?}", token),
        }
    }

    /**
     * arguments -> expression ( "," expression )* ;
     */
    fn arguments(&mut self) -> Vec<Expression> {
        let mut arguments = Vec::new();

        if self.lexer.peek_token() != Token::Rparen {
            loop {
                if arguments.len() >= 255 {
                    panic!("Cannot have more than 255 arguments");
                }

                arguments.push(self.expression());

                if !self.lexer.match_token_and_consume(Token::Comma) {
                    break;
                }
            }
        }

        return arguments;
    }

    fn finish_call(&mut self, callee: Expression) -> Expression {
        let arguments = self.arguments();

        self.expect(Token::Rparen, "Expected a closing parenthesis");

        return Expression::call(callee, arguments);
    }

    /**
     * call -> primary ( "(" arguments? ")" )* ;
     */
    fn call(&mut self) -> Expression {
        let mut expr = self.primary();

        while self.lexer.match_token_and_consume(Token::Lparen) {
            expr = self.finish_call(expr);
        }

        return expr;
    }

    /**
     * unary -> ( "!" | "-" ) unary | call ;
     */
    fn unary(&mut self) -> Expression {
        match self.lexer.peek_token() {
            Token::Bang | Token::Minus => {
                let token = self.lexer.next_token();
                let operator = self.parse_token_to_operator(token);
                let right = self.unary();

                return Expression::unary(operator, right);
            }
            _ => return self.call(),
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
     * assignment -> IDENTIFIER "=" assignment | logic_or ;
     */
    fn assignment(&mut self) -> Expression {
        let expr = self.or();

        if self.lexer.match_token_and_consume(Token::Assign) {
            let ident = match expr {
                Expression::Variable(ident) => ident,
                _ => panic!("Expected an identifier"),
            };

            let value = self.assignment();

            return Expression::assignement(ident, value);
        }

        return expr;
    }

    /**
     * logic_or -> logic_and ( "or" logic_and )* ;
     */
    fn or(&mut self) -> Expression {
        let mut expr = self.and();

        while self.lexer.match_token_and_consume(Token::Or) {
            let operator = Operator::Or;
            let right = self.and();

            expr = Expression::binary(expr, operator, right);
        }

        return expr;
    }

    /**
     * logic_and -> equality ( "and" equality )* ;
     */
    fn and(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.lexer.match_token_and_consume(Token::And) {
            let operator = Operator::And;
            let right = self.equality();

            expr = Expression::binary(expr, operator, right); // should we create Expression::logical?
        }

        return expr;
    }

    /**
     * expression -> assignment ;
     */
    fn expression(&mut self) -> Expression {
        if self.lexer.match_token_and_consume(Token::Function) {
            return self.function();
        }

        return self.assignment();
    }

    fn parse_ident(&mut self) -> Ident {
        match self.lexer.next_token() {
            Token::Ident(ident) => return Ident::new(ident),
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

    fn expect(&mut self, token: Token, message: &str) {
        if !self.lexer.match_token_and_consume(token) {
            panic!("{}, got {:?}", message, self.lexer.next_token());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{s, statements::block::BlockStatement};

    use super::*;

    #[test]
    fn let_statement() {
        let mut parser = Parser::new(s!("let a = 1;"));
        let stmt = parser.parse();

        assert_eq!(
            stmt,
            vec![Statement::_let(
                Ident::new("a"),
                Some(Expression::literal(ParserValue::number("1")))
            )]
        );
    }

    #[test]
    fn literal_expression() {
        let mut parser = Parser::new(s!("1;"));
        let expr = parser.expression();

        assert_eq!(expr, Expression::literal(ParserValue::number("1")));
    }

    #[test]
    fn binary_expression() {
        let mut parser = Parser::new(s!("1 + 2;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::binary(
                Expression::literal(ParserValue::number("1")),
                Operator::Plus,
                Expression::literal(ParserValue::number("2")),
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
                Expression::literal(ParserValue::number("1")),
                Operator::Plus,
                Expression::literal(ParserValue::number("2")),
            ))
        );
    }

    #[test]
    fn unary_expression() {
        let mut parser = Parser::new(s!("!true;"));
        let expr = parser.expression();

        assert_eq!(
            expr,
            Expression::unary(Operator::Bang, Expression::literal(ParserValue::Bool(true)))
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
                    Expression::literal(ParserValue::Bool(true))
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
                    Expression::unary(Operator::Bang, Expression::literal(ParserValue::Bool(true))),
                    Operator::Plus,
                    Expression::literal(ParserValue::number("1")),
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
                Expression::literal(ParserValue::number("1")),
                Operator::Plus,
                Expression::binary(
                    Expression::literal(ParserValue::number("2")),
                    Operator::Asterisk,
                    Expression::literal(ParserValue::number("3")),
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
                    Expression::literal(ParserValue::number("1")),
                    Operator::Plus,
                    Expression::literal(ParserValue::number("2")),
                )),
                Operator::Asterisk,
                Expression::literal(ParserValue::number("3")),
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
            assert_eq!(stmt, Statement::_let(Ident::new("a"), None,));
        }
    }

    #[test]
    fn let_statement_initialized() {
        let mut parser = Parser::new(s!("let a = 1;"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::_let(
                    Ident::new("a"),
                    Some(Expression::Literal(ParserValue::number("1"))),
                )
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
    //             expression: Some(Expression::Literal(Value::number("1"))),
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
                Statement::Expression(Expression::Literal(ParserValue::number("1")))
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
                Statement::_block(vec![Statement::_expression(Expression::literal(
                    ParserValue::number("1")
                ))])
            );
        }
    }

    #[test]
    fn empty_block_statement() {
        let mut parser = Parser::new(s!("{ }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(stmt, Statement::_block(vec![]));
        }
    }

    #[test]
    fn if_statement() {
        let mut parser = Parser::new(s!("if (true) { 1; }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::_if(
                    Expression::Literal(ParserValue::Bool(true)),
                    Statement::_block(vec![Statement::Expression(Expression::Literal(
                        ParserValue::number("1")
                    ))],),
                    None,
                )
            );
        }
    }

    #[test]
    fn function_statement() {
        let mut parser = Parser::new(s!("function a() { let b = 1; }"));
        let stmt = parser.parse();

        for stmt in stmt {
            assert_eq!(
                stmt,
                Statement::function(
                    Ident::new("a"),
                    vec![],
                    BlockStatement::new(vec![Statement::_let(
                        Ident::new("b"),
                        Some(Expression::Literal(ParserValue::number("1")))
                    ),],),
                )
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
    //                 condition: Expression::Literal(Ident::new("a")),
    //                 consequence: Box::new(Statement::Block(BlockStatement(vec![],))),
    //                 alternative: Some(Box::new(Statement::Block(BlockStatement(vec![
    //                     Statement::Expression(Expression::Assign {
    //                         ident: Ident::new("a"),
    //                         expression: Box::new(Expression::Literal(Value::Bool(true))),
    //                     })
    //                 ],)))),
    //             })
    //         );
    //     }
    // }

    #[test]
    fn return_statement() {
        let mut parser = Parser::new(s!("return 1; return; return a;"));
        let stmt = parser.parse();

        let expected = vec![
            Statement::_return(Expression::literal(ParserValue::number("1"))),
            Statement::_return(Expression::literal(ParserValue::Null)),
            Statement::_return(Expression::variable("a")),
        ];

        assert_eq!(stmt, expected);
    }

    #[test]
    fn function_with_closures() {
        let mut parser = Parser::new(s!("function makeCounter() {
            let i = 0;
            
            function count() {
                i = i + 1;
                print i; 
            }
        
            return count;
        }"));
        let stmt = parser.parse();

        let expected = vec![Statement::function(
            Ident::new("makeCounter"),
            vec![],
            BlockStatement::new(vec![
                Statement::_let(
                    Ident::new("i"),
                    Some(Expression::literal(ParserValue::number("0"))),
                ),
                Statement::function(
                    Ident::new("count"),
                    vec![],
                    BlockStatement::new(vec![
                        Statement::_expression(Expression::assignement(
                            Ident::new("i"),
                            Expression::binary(
                                Expression::variable("i"),
                                Operator::Plus,
                                Expression::literal(ParserValue::number("1")),
                            ),
                        )),
                        Statement::print(Expression::variable("i")),
                    ]),
                ),
                Statement::_return(Expression::variable("count")),
            ]),
        )];

        assert_eq!(stmt, expected);
    }
}
