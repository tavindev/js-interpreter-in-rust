use crate::lexer::lexer::Lexer;

enum Expression {}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn new(input: String) -> Parser {
        let mut parser = Parser {
            lexer: Lexer::new(input),
        };

        return parser;
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
