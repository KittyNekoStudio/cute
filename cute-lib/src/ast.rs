trait Ast {}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Add(Box<Add>),
}

#[derive(Debug, PartialEq)]
pub struct Number(pub u64);

impl Ast for Number {}

#[derive(Debug, PartialEq)]
pub struct Add {
    lhs: Expression,
    rhs: Expression,
}

impl Add {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        Self { lhs, rhs }
    }
}

impl Ast for Add {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_new() {
        assert_eq!(
            Add::new(
                Expression::Number(Number(400)),
                Expression::Number(Number(20))
            ),
            Add {
                lhs: Expression::Number(Number(400)),
                rhs: Expression::Number(Number(20)),
            }
        );
    }
}
