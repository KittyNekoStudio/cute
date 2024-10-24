use std::fs::File;
use std::io::prelude::*;

const VEC_STARTING_SIZE: usize = 64000;

#[derive(Debug, PartialEq)]
struct Number(i32);

#[derive(Debug, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
enum Value {
    Number(Number),
}

#[derive(Debug, PartialEq)]
struct Expression {
    lhs: Number,
    op: Operation,
    rhs: Number,
}

#[derive(Debug, PartialEq, Clone)]
struct Buffer(Vec<String>);

impl Expression {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, lhs) = Number::new(string);
        let (string, op) = Operation::new(string);
        let (string, rhs) = Number::new(string);

        (
            string,
            Self {
                rhs: rhs,
                op: op,
                lhs: lhs,
            },
        )
    }
}

impl Number {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, number) = extract_number(string);
        (string, Self(number.parse().unwrap()))
    }
}

impl Operation {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, op) = extract_operation(string);
        (string, op)
    }
}

impl Buffer {
    pub fn new() -> Self {
        // TODO! change vec to somthing more efficient
        Self(Vec::with_capacity(VEC_STARTING_SIZE))
    }
    pub fn push(&mut self, string: &str) {
        for str in string.lines() {
            self.0.push(str.to_string())
        }
    }
}

fn extract_operation(string: &str) -> (&str, Operation) {
    let (string, _) = extract_whitespace(string);
    let op = match &string[0..1] {
        "+" => Operation::Addition,
        "-" => Operation::Subtraction,
        "*" => Operation::Multiplication,
        "/" => Operation::Division,
        _ => panic!("unreachable"),
    };
    let string = &string[1..];

    (string, op)
}

fn extract_number(string: &str) -> (&str, &str) {
    let (string, _) = extract_whitespace(string);
    let number_end = string
        .char_indices()
        .find_map(|(index, char)| {
            if char.is_ascii_digit() {
                return None;
            } else {
                return Some(index);
            }
        })
        .unwrap_or_else(|| string.len());

    let number = &string[..number_end];
    let remainder = &string[number_end..];
    (remainder, number)
}

fn extract_whitespace(string: &str) -> (&str, &str) {
    let whitespace_end = string
        .char_indices()
        .find_map(|(index, char)| {
            if char.is_whitespace() {
                return None;
            } else {
                return Some(index);
            }
        })
        .unwrap_or_else(|| string.len());

    let remainder = &string[whitespace_end..];
    let whitespace = &string[..whitespace_end];
    (remainder, whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_whitespace() {
        assert_eq!(Number::new("   5"), ("", Number(5)));
    }
    #[test]
    fn test_buffer_push() {
        let mut buffer = Buffer::new();
        buffer.push("1 + 3\n3 * 3\n");

        assert_eq!(buffer.0, vec!["1 + 3", "3 * 3"]);
    }
    #[test]
    fn test_string_from_file() {
        let mut file = File::open("src/test.cute").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut buffer = Buffer::new();
        buffer.push(&contents);

        assert_eq!(buffer.0, vec!["1 + 3", "3 * 3"]);
    }
}
