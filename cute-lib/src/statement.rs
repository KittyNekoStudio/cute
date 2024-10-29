use crate::expression::Expression;
use std::fs::File;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expr(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

pub trait GenerateAsm {
    fn generate(&self, file: &mut File);
}

impl GenerateAsm for BlockStatement {
    fn generate(&self, file: &mut File) {
        for stmt in self.body.clone() {
            stmt.generate(file)
        }
    }
}
impl GenerateAsm for Statement {
    fn generate(&self, file: &mut File) {
        match self {
            Statement::Expr(expr) => expr.generate(file),
        }
    }
}
impl GenerateAsm for ExpressionStatement {
    fn generate(&self, file: &mut File) {
        self.expression.generate(file)
    }
}
