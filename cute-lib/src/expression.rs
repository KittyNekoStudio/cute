use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(Number),
    Symbol(Symbol),
    Binary(Box<BinaryExpression>),
    Write
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
    pub fn get_lhs(&self) -> &Expression {
        &self.lhs
    }
    pub fn get_op(&self) -> &Token {
        &self.op
    }
    pub fn get_rhs(&self) -> &Expression {
        &self.rhs
    }
}
