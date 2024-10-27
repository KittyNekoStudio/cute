use crate::expression::Expression;

pub enum Statement {

}

pub struct BlockStatement {
    pub body: Vec<Statement>
}

pub struct ExpressionStatement {
    expression: Expression
}
