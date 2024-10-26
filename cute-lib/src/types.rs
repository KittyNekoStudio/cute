use crate::utils::*;
#[derive(Debug, PartialEq, Clone)]
pub struct Number(pub i32);

impl Number {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, _) = extract_whitespace(string);
        let (string, number) = extract_number(string);
        (string, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, op) = extract_operation(string);
        (string, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_whitespace() {
        assert_eq!(Number::new("   5"), ("", Number(5)));
    }
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("5"), ("", Number(5)));
    }
    #[test]
    fn parse_add() {
        assert_eq!(Operation::new("+"), ("", Operation::Addition));
    }
    #[test]
    fn parse_sub() {
        assert_eq!(Operation::new("-"), ("", Operation::Subtraction));
    }
    #[test]
    fn parse_mul() {
        assert_eq!(Operation::new("*"), ("", Operation::Multiplication));
    }
    #[test]
    fn parse_div() {
        assert_eq!(Operation::new("/"), ("", Operation::Division));
    }
}
