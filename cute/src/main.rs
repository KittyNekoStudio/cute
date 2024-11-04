use cute_lib::utils::remove_file_extension;
use cute_lib::file::read_file;
use cute_lib::parser::*;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = match args[1].split(".").collect::<Vec<&str>>()[1] {
        "cute" => remove_file_extension(&args[1]),
        _ => panic!("File not doesn't have .cute extension"),
    };
    let source = read_file(path);
    let mut parser = Parser::new(source);
    let expressions = parse_expressions(&mut parser);
    //println!("{parser:?}");
    println!("{expressions:?}");
}
