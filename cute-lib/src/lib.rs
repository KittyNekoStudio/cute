mod token;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const VEC_STARTING_SIZE: usize = 64000;

#[derive(Debug, PartialEq, Clone)]
struct Number(i32);

#[derive(Debug, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

// TODO! find a way to use value
#[derive(Debug, PartialEq, Clone)]
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
pub struct Buffer(Vec<String>);

#[derive(Debug, PartialEq, Clone)]
struct Binding {
    name: String,
    value: Value,
}

#[derive(Debug, PartialEq)]
struct Environment {
    env: HashMap<String, Value>,
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

impl Number {
    pub fn new(string: &str) -> (&str, Self) {
        let (string, _) = extract_whitespace(string);
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

impl Binding {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        let string = extract_keyword(string, "let")?;
        let (string, _) = extract_whitespace(string);

        let (string, name) = extract_name(string)?;
        let (string, _) = extract_whitespace(string);

        println!("{string}");
        let string = extract_keyword(string, "=")?;
        let (string, _) = extract_whitespace(string);

        let (string, expr) = Expression::new(string);
        Ok((
            string,
            Self {
                name: name.to_string(),
                value: expr.eval(),
            },
        ))
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
    fn get_binding(&self, key: &str) -> Value {
        self.env.get(key).cloned().unwrap()
    }
    fn store_binding(&mut self, binding: Binding) {
        self.env.insert(binding.name.to_string(), binding.value);
    }
}

fn extract(accept: impl Fn(char) -> bool, string: &str) -> (&str, &str) {
    let extracted_end = string
        .char_indices()
        .find_map(|(index, char)| {
            if accept(char) {
                return None;
            } else {
                return Some(index);
            }
        })
        .unwrap_or(string.len());

    let remainder = &string[extracted_end..];
    let extracted = &string[..extracted_end];
    (remainder, extracted)
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
    extract(|char| char.is_ascii_digit(), string)
}

fn extract_whitespace(string: &str) -> (&str, &str) {
    extract(|char| char.is_whitespace(), string)
}

fn extract_until_whitespace(string: &str) -> (&str, &str) {
    extract(|char| !char.is_whitespace(), string)
}

fn extract_keyword<'a>(string: &'a str, starting_string: &str) -> Result<&'a str, String> {
    if string.starts_with(starting_string) {
        return Ok(&string[starting_string.len()..]);
    } else {
        return Err(format! {"Error extracting keyword"});
    }
}

fn extract_name(string: &str) -> Result<(&str, &str), String> {
    Ok(extract(|char| !char.is_whitespace(), string))
}

fn convert_strings_to_expressions(buffer: Buffer) -> Vec<Expression> {
    let mut vec = Vec::new();
    for v in buffer.0 {
        vec.push(Expression::new(&v).1);
    }
    vec
}

fn read_to_file(path: &str, buffer: &mut Buffer) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    buffer.push(&contents);
}

pub fn write_to_file(path: &str, buffer: &mut Buffer) {
    read_to_file(path, buffer);
    // TODO! remove having to clone buffer
    let expressions = convert_strings_to_expressions(buffer.to_owned());
    let dot = path
        .char_indices()
        .find_map(|(index, char)| {
            if char == '.' {
                return Some(index);
            } else {
                return None;
            }
        })
        .unwrap_or_else(|| path.len());

    let path = &path[..dot];

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{path}.asm"))
        .unwrap();
    // TODO! add comments to asm
    write!(file, "    global print_uint32\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "print_uint32:\n").unwrap();
    write!(file, "    mov rax, rdi\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "    mov  ecx, 0xa\n").unwrap();
    write!(file, "    push rcx\n").unwrap();
    write!(file, "    mov  rsi, rsp\n").unwrap();
    write!(file, "    sub  rsp, 16\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "    .toascii_digit:\n").unwrap();
    write!(file, "    xor  edx, edx\n").unwrap();
    // TODO! div is slow
    write!(file, "    div  ecx\n").unwrap();
    write!(file, "    add  edx, '0'\n").unwrap();
    write!(file, "    dec  rsi\n").unwrap();
    write!(file, "    mov [rsi], dl\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "    test rax, rax\n").unwrap();
    write!(file, "    jnz  .toascii_digit\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "    mov  rax, 1\n").unwrap();
    write!(file, "    mov  rdi, 1\n").unwrap();
    write!(file, "    lea  edx, [rsp+16 + 1]\n").unwrap();
    write!(file, "    sub  edx, esi\n").unwrap();
    write!(file, "    syscall\n").unwrap();
    write!(file, "    add  rsp, 24\n").unwrap();
    write!(file, "    ret\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "section .text\n").unwrap();
    write!(file, "    global _start\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "_start:\n").unwrap();

    for expr in expressions {
        match expr.op {
            Operation::Addition => {
                let lhs = expr.lhs.0;
                let rhs = expr.rhs.0;
                write!(file, "    mov  rdi, {lhs}\n").unwrap();
                write!(file, "    add  rdi, {rhs}\n").unwrap();
                write!(file, "    call print_uint32\n").unwrap();
            }
            Operation::Subtraction => {
                let lhs = expr.lhs.0;
                let rhs = expr.rhs.0;
                write!(file, "    mov  rdi, {lhs}\n").unwrap();
                write!(file, "    sub  rdi, {rhs}\n").unwrap();
                write!(file, "    call print_uint32\n").unwrap();
            }
            Operation::Multiplication => {
                let lhs = expr.lhs.0;
                let rhs = expr.rhs.0;
                write!(file, "    mov  rax, {lhs}\n").unwrap();
                write!(file, "    mov  rdx, {rhs}\n").unwrap();
                write!(file, "    mul  rdx\n").unwrap();
                write!(file, "    mov  rdi, rax \n").unwrap();
                write!(file, "    call print_uint32\n").unwrap();
            }
            Operation::Division => {
                assert!(false, "Division not implemented")
            }
        }
    }
    write!(file, "    mov  rax, 60\n").unwrap();
    write!(file, "    mov  rdi, 0\n").unwrap();
    write!(file, "    syscall\n").unwrap();
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
    fn parse_binding_value_from_expression() {
        assert_eq!(
            Binding::new("let i = 1 + 8").unwrap().1,
            Binding {
                name: "i".to_string(),
                value: Value::Number(Number(9))
            }
        );
    }
    /*#[test]
    fn parse_binding_value_from_int() {
        assert_eq!(
            Binding::new("let i = 10").unwrap().1,
            Binding {
                name: "i".to_string(),
                value: Value::Number(Number(10))
            }
        );
    }*/
    #[test]
    fn evaluate_number() {
        assert_eq!(Expression::new("4 / 2").1.eval(), Value::Number(Number(2)));
    }
    #[test]
    fn buffer_push() {
        let mut buffer = Buffer::new();
        buffer.push("1 + 3\n3 * 3\n");

        assert_eq!(buffer.0, vec!["1 + 3", "3 * 3"]);
    }
    #[test]
    fn string_from_file() {
        let mut buffer = Buffer::new();
        read_to_file("tests/test-string.cute", &mut buffer);

        assert_eq!(buffer.0, vec!["1 + 3"]);
    }
    #[test]
    fn expression_from_file() {
        let mut buffer = Buffer::new();
        read_to_file("tests/test-expression.cute", &mut buffer);
        assert_eq!(
            Expression::new(&buffer.0[0]),
            (
                "",
                Expression {
                    lhs: Number(4),
                    op: Operation::Subtraction,
                    rhs: Number(2)
                }
            )
        );
    }
    #[test]
    fn test_write_to_file() {
        let mut buffer = Buffer::new();
        write_to_file("tests/test.cute", &mut buffer);
    }
    #[test]
    fn evaluate_binded_value() {
        let mut environment = Environment::new();
        let binding = Binding::new("let i = 10 + 3").unwrap().1;
        environment.store_binding(binding.clone());
        assert_eq!(
            environment.get_binding(&binding.name),
            Value::Number(Number(13))
        );
    }
}
