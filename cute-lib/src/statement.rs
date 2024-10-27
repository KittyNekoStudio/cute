use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expr(ExpressionStatement)
}

#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}
