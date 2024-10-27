use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(Number),
    Symbol(Symbol),
    Binary(Box<BinaryExpression>),
}

// Literals
#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub f64);
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(pub String);

// Complex expressions
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    lhs: Expression,
    op: Token,
    rhs: Expression,
}

impl BinaryExpression {
    pub fn new(lhs: Expression, op: Token, rhs: Expression) -> Self {
        Self { lhs, op, rhs }
    }
}
