use crate::parser::{expression::Expression, ident::Ident};

use super::{block::BlockStatement, r#if::IfStatement, r#let::LetStatement};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    Block(BlockStatement),
    Expression(Expression),
}

impl Statement {
    pub fn _let(ident: Ident, expression: Option<Expression>) -> Self {
        Self::Let(LetStatement { ident, expression })
    }

    pub fn _if(
        condition: Expression,
        consequence: Statement,
        alternative: Option<Statement>,
    ) -> Self {
        if let Statement::Let(_) = consequence {
            panic!("consequence cannot be a let statement")
        }

        Self::If(IfStatement {
            condition,
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        })
    }

    pub fn _block(statements: Vec<Statement>) -> Self {
        Self::Block(BlockStatement::new(statements))
    }

    pub fn _expression(expression: Expression) -> Self {
        Self::Expression(expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::value::Value;

    use super::*;

    fn expression() -> Expression {
        return Expression::assignement(Ident::new("x"), Expression::literal(Value::number(1)));
    }

    #[test]
    #[should_panic]
    fn test_if_with_let() {
        Statement::_if(
            expression(),
            Statement::_let(Ident::new("x"), Some(expression())),
            None,
        );
    }
}
