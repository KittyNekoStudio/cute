use cute_lib::file::read_to_file;
use cute_lib::lexer::tokenize;
use cute_lib::parser::parse;
fn main() {
    let source = read_to_file("foo.cute");
    let tokens = tokenize(source);
    let statement = parse(tokens);
    println!("{statement:?}");
}
