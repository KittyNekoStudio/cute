use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expr(ExpressionStatement)
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}
