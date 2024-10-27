use crate::lexer::Token;

pub enum Expression {
    Number(f64),
    Symbol(String),
    Binary(Box<BinaryExpression>)
}

// Literals
pub struct Number(f64);
pub struct Symbol(String);

// Complex expressions
pub struct BinaryExpression {
    lhs: Expression,
    op: Token,
    rhs: Expression
}
