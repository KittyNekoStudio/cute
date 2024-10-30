use std::env::args;
use cute_lib::file::generate_x86_64_linux_asm;
use cute_lib::utils::{compile_and_link, remove_file_extension};
use cute_lib::file::read_to_file;
use cute_lib::lexer::tokenize;
use cute_lib::parser::parse;
fn main() {
    let args: Vec<String> = args().collect();
    let path: String = 
        match args[1].split(".").collect::<Vec<&str>>()[1] {
            "cute" => remove_file_extension(&args[1]),
            _ => panic!("File not doesn't have .cute extension")
        };
    let source = read_to_file(path.to_string());
    let tokens = tokenize(source);
    let statements = parse(tokens);
    println!("{statements:?}");
    println!("{}", statements.body.len());
    generate_x86_64_linux_asm(&path);
    compile_and_link(&path);
}
