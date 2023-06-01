use crate::parser::parser::Parser;

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new(input: String) -> Interpreter {
        Interpreter {
            parser: Parser::new(input),
        }
    }

    /**
     * Eval each expression
     */
    pub fn run(&mut self) {
        let expressions = self.parser.parse();

        for expression in expressions.iter().cloned() {
            println!("{:?}", expression);
        }
    }
}

#[cfg(test)]
mod tests {}
