use cute_lib::file::read_to_file;
use cute_lib::lexer::tokenize;
use cute_lib::parser::parse;
use cute_lib::file::generate_x86_64_linux_asm;
fn main() {
    let source = read_to_file("foo.cute");
    let tokens = tokenize(source);
    let statements = parse(tokens);
    println!("{statements:?}");
    println!("{}", statements.body.len());
    generate_x86_64_linux_asm("foo.cute"); 
    
}
