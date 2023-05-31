#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
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
                    _ => Token::Ident(ident),
                };
            }
            b'0'..=b'9' => return Token::Int(self.read_int()),
            b'\n' => {
                if self.peek_char() == b'\r' {
                    self.read_char();
                }

                Token::Newline
            }
            0 => Token::Eof,
            _ => todo!("we need to implement this...."),
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
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

    fn read_int(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_digit() {
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
            Token::Ident(String::from("s")),
            Token::Assign,
            Token::String(String::from("hello world")),
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn read_int() {
        let input = r#"123;"#;
        let mut lex = Lexer::new(input.into());

        assert_eq!(lex.next_token(), Token::Int("123".into()));
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
            println!("expected: {:?}, received {:?}", token, next_token);
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
            "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::LSquirly,
            Token::Return,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::ForwardSlash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreaterThan,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
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
            Token::Int(String::from("10")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEqual,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::LessThanOrEqual,
            Token::GreaterThanOrEqual,
            Token::And,
            Token::Or,
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
        assert_eq!(Token::Ident(String::from("five")), lex.peek_token());
        assert_eq!(Token::Let, lex.curr_token());
    }
}
