#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(String),
    String(String),
    Print, // temporary
    Null,
    Illegal,
    Eof,
    Bang,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Plus,
    Minus,
    Asterisk,
    And,
    Or,
    ForwardSlash,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
    If,
    Else,
    While,
    For,
    Do,
    Return,
    True,
    False,
    Newline,
}

impl Token {
    pub fn string<S: Into<String>>(string: S) -> Self {
        Token::String(string.into())
    }

    pub fn number<S: Into<String>>(number: S) -> Self {
        Token::Number(number.into())
    }

    pub fn ident<S: Into<String>>(ident: S) -> Self {
        Token::Ident(ident.into())
    }
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
    curr_token: Token,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
            curr_token: Token::Illegal,
        };

        lex.read_char();

        return lex;
    }

    /**
     * Early returns solves a bug where the lexer would read a char when it shouldn't
     */
    fn parse_token(&mut self) -> Token {
        let token = match self.ch {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::ForwardSlash,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::Illegal
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    Token::Or
                } else {
                    Token::Illegal
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'"' => {
                // dont know if is the ideal solution lmao
                self.read_char(); // skip the first "

                let string = self.read_delimiter(b'"');

                self.read_char(); // skip the last "

                return Token::String(string);
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();

                return match ident.as_str() {
                    "function" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "do" => Token::Do,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    "null" => Token::Null,
                    "print" => Token::Print, // temporary
                    _ => Token::Ident(ident),
                };
            }
            // FIX: Reads , as a number literal
            b'0'..=b'9' | b'.' => return Token::Number(self.read_number()),
            b'\n' => {
                if self.peek_char() == b'\r' {
                    self.read_char();
                }

                Token::Newline
            }
            0 => Token::Eof,
            _ => panic!("Unexpected character: {}", self.ch as char),
        };

        self.read_char();

        return token;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = self.parse_token();
        self.curr_token = token.clone();

        return token;
    }

    pub fn peek_char(&self) -> u8 {
        if self.position + 1 >= self.input.len() {
            return 0;
        } else {
            return self.input[self.position + 1];
        }
    }

    pub fn match_token_and_consume(&mut self, token: Token) -> bool {
        if self.peek_token() == token {
            self.next_token();
            return true;
        }

        return false;
    }

    // dont know how I feel about this method
    pub fn peek_token(&mut self) -> Token {
        let pos = self.position;
        let read_pos = self.read_position;
        let ch = self.ch;
        let current_token = self.curr_token.clone();

        let token = self.next_token();
        self.position = pos;
        self.read_position = read_pos;
        self.ch = ch;
        self.curr_token = current_token;

        token
    }

    pub fn curr_token(&self) -> Token {
        return self.curr_token.clone();
    }

    pub fn is_at_end(&self) -> bool {
        return self.read_position >= self.input.len();
    }

    fn read_char(&mut self) {
        if self.is_at_end() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_delimiter(&mut self, delimiter: u8) -> String {
        let pos = self.position;

        while self.ch != delimiter {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        let mut has_dot = false;

        while self.ch.is_ascii_digit() || (self.ch == b'.' && !has_dot) {
            if self.ch == b'.' {
                has_dot = true;
            }

            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {

    use super::{Lexer, Token};

    #[test]
    fn read_delimiter() {
        let input = r#"let s = "hello world";"#;
        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::ident("s"),
            Token::Assign,
            Token::string("hello world"),
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn read_int() {
        let input = r#"123;"#;
        let mut lex = Lexer::new(input.into());

        assert_eq!(lex.next_token(), Token::number("123"));
        assert_eq!(lex.next_token(), Token::Semicolon);
    }

    #[test]
    fn get_next_token() {
        let input = "=+(){},;!===";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
            Token::NotEqual,
            Token::Equal,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            assert_eq!(token, next_token);
            assert_eq!(next_token, lexer.curr_token());
        }
    }

    #[test]
    fn get_next_complete() {
        let input = r#"let add = function(x, y) {
                return x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            <=
            >=
            &&
            ||
            .51;
            1.23;
            2.3.4;
            "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::ident("add"),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::ident("x"),
            Token::Comma,
            Token::ident("y"),
            Token::Rparen,
            Token::LSquirly,
            Token::Return,
            Token::ident("x"),
            Token::Plus,
            Token::ident("y"),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            Token::Let,
            Token::ident("result"),
            Token::Assign,
            Token::ident("add"),
            Token::Lparen,
            Token::ident("five"),
            Token::Comma,
            Token::ident("ten"),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::ForwardSlash,
            Token::Asterisk,
            Token::number("5"),
            Token::Semicolon,
            Token::number("5"),
            Token::LessThan,
            Token::number("10"),
            Token::GreaterThan,
            Token::number("5"),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::number("5"),
            Token::LessThan,
            Token::number("10"),
            Token::Rparen,
            Token::LSquirly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RSquirly,
            Token::Else,
            Token::LSquirly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RSquirly,
            Token::number("10"),
            Token::Equal,
            Token::number("10"),
            Token::Semicolon,
            Token::number("10"),
            Token::NotEqual,
            Token::number("9"),
            Token::Semicolon,
            Token::LessThanOrEqual,
            Token::GreaterThanOrEqual,
            Token::And,
            Token::Or,
            Token::number(".51"),
            Token::Semicolon,
            Token::number("1.23"),
            Token::Semicolon,
            Token::number("2.3"),
            Token::number(".4"),
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn peek_token() {
        let input = "let five = 5;";

        let mut lex = Lexer::new(input.into());

        assert_eq!(Token::Let, lex.peek_token());
        assert_eq!(Token::Illegal, lex.curr_token());
        assert_eq!(Token::Let, lex.next_token());
        assert_eq!(Token::ident("five"), lex.peek_token());
        assert_eq!(Token::Let, lex.curr_token());
    }

    #[test]
    fn match_token() {
        let input = "let five = 5;";

        let mut lex = Lexer::new(input.into());

        assert_eq!(lex.match_token_and_consume(Token::Let), true);
        assert_eq!(lex.match_token_and_consume(Token::Let), false);
    }
}
