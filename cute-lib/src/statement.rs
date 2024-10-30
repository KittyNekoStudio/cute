use crate::expression::Expression;

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
