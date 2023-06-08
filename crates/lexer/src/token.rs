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
