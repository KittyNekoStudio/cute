use crate::lexer::Token;
use crate::statement::GenerateAsm;
use std::fs::File;
use std::io::Write;

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
impl GenerateAsm for Expression {
    fn generate(&self, file: &mut File) {
        match self {
            Expression::Number(num) => num.generate(file),
            Expression::Binary(num) => num.generate(file),
            Expression::Symbol(num) => num.generate(file),
        }
    }
}
impl GenerateAsm for BinaryExpression {
    fn generate(&self, _file: &mut File) {}
}
impl GenerateAsm for Number {
    fn generate(&self, file: &mut File) {
        let num = match self {
            Number(num) => num,
        };
        write!(file, "    mov  rdi, {num}\n").unwrap();
        write!(file, "    add  rdi, {num}\n").unwrap();
        write!(file, "    call print_uint32\n").unwrap();
    }
}
impl GenerateAsm for Symbol {
    fn generate(&self, _file: &mut File) {}
}
