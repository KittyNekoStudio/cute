#[derive(Debug, PartialEq)]
/// Expression
/// All the different expressions cute supports.
pub enum Expression {
    Number(Number),
    Add(Box<Add>),
}

#[derive(Debug, PartialEq)]
/// Number
/// An unsigned 64 bit number.
pub struct Number(pub u64);

#[derive(Debug, PartialEq)]
/// Add
/// An expression that adds two expressions together.
pub struct Add {
    lhs: Expression,
    rhs: Expression,
}

impl Add {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        Self { lhs, rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_add() {
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
