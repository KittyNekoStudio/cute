use crate::lexer::Token;

pub enum Expression<'a>{
    Number(f64),
    Symbol(String),
    Binary(&'a BinaryExpression<'a>)
}

// Literals
pub struct Number(f64);
pub struct Symbol(String);

// Complex expressions
pub struct BinaryExpression<'a> {
    lhs: Expression<'a>,
    op: Token,
    rhs: Expression<'a>
}
