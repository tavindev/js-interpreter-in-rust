use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
    Illegal,
    Eof,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
    Return,
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        return lex;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'=' => Token::Equal,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'"' => {
                self.read_char(); // skip the first "

                let string = self.read_delimiter(b'"');
                return Ok(Token::String(string));
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "function" => Token::Function,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
            0 => Token::Eof,
            _ => todo!("we need to implement this...."),
        };

        self.read_char();
        return Ok(tok);
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
            println!("{}", self.ch as char);
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
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn read_delimiter() -> Result<()> {
        let input = r#""hello world""#;
        let mut lexer = Lexer::new(input.into());

        let tok = lexer.next_token()?;
        assert_eq!(tok, Token::String(String::from("hello world")));

        return Ok(());
    }

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Equal,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn get_next_complete() -> Result<()> {
        let input = r#"let five = 5;
						let str = "hello";
            let ten = 10;
            let add = function(x, y) {
                return x + y;
            };
            let result = add(five, ten);"#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Equal,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("str")),
            Token::Equal,
            Token::String(String::from("hello")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Equal,
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
            Token::Equal,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lex.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
