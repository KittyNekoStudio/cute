use crate::lexer::tokenize;
use crate::parser::parse;
use crate::utils::remove_file_extension;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

pub fn read_to_file(path: String) -> Vec<String> {
    let file = File::open(format!("{path}.cute")).expect("No such file found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn generate_x86_64_linux_asm(path: &str) {
    let path = remove_file_extension(path);
    let source = read_to_file(path.to_string());
    let tokens = tokenize(source);
    let _statements = parse(tokens);

    // TODO! remove having to clone buffer
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{path}.asm"))
        .unwrap();

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
    /*match stmt.op {
            Operation::Addition => {
                let lhs = stmt.lhs.0;
                let rhs = stmt.rhs.0;
                write!(file, "    mov  rdi, {lhs}\n").unwrap();
                write!(file, "    add  rdi, {rhs}\n").unwrap();
                write!(file, "    call print_uint32\n").unwrap();
            }
            Operation::Subtraction => {
                let lhs = stmt.lhs.0;
                let rhs = stmt.rhs.0;
                write!(file, "    mov  rdi, {lhs}\n").unwrap();
                write!(file, "    sub  rdi, {rhs}\n").unwrap();
                write!(file, "    call print_uint32\n").unwrap();
            }
            Operation::Multiplication => {
                let lhs = stmt.lhs.0;
                let rhs = stmt.rhs.0;
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
    }*/
    write!(file, "    call print_uint32\n").unwrap();
    write!(file, "    mov  rax, 60\n").unwrap();
    write!(file, "    mov  rdi, 0\n").unwrap();
    write!(file, "    syscall\n").unwrap();
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;

    #[test]
    fn string_from_file() {
        let buffer = read_to_file("tests/test-string.cute");

        assert_eq!(buffer, vec!["1 + 3"]);
    }
    #[test]
    fn expression_from_file() {
        let mut buffer = read_to_file("tests/test-expression.cute", &mut buffer);
        assert_eq!(
            Expression::new(&buffer[0]),
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
        let mut buffer = Vec::new();
        write_to_file("tests/test.cute", &mut buffer);
    }
}*/
