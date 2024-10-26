use crate::types::*;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Number,
    pub op: Operation,
    pub rhs: Number,
}

impl Expression {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, lhs) = Number::new(string);
        let (string, op) = Operation::new(string);
        let (string, rhs) = Number::new(string);

        (string, Self { rhs, op, lhs })
    }
    pub fn eval(&self) -> Value {
        match self.op {
            // TODO! clean up the 0 indexing into number type
            Operation::Addition => Value::Number(Number(self.lhs.0 + self.rhs.0)),
            Operation::Subtraction => Value::Number(Number(self.lhs.0 - self.rhs.0)),
            Operation::Multiplication => Value::Number(Number(self.lhs.0 * self.rhs.0)),
            Operation::Division => Value::Number(Number(self.lhs.0 / self.rhs.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_expression() {
        assert_eq!(
            Expression::new("5 + 3"),
            (
                "",
                Expression {
                    lhs: Number(5),
                    op: Operation::Addition,
                    rhs: Number(3)
                }
            )
        );
    }
    #[test]
    fn evaluate_number() {
        assert_eq!(Expression::new("4 / 2").1.eval(), Value::Number(Number(2)));
    }
}
