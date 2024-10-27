use cute_lib::file::read_to_file;
use cute_lib::lexer::tokenize;
fn main() {
    let mut source = Vec::new();
    read_to_file("foo.cute", &mut source);
    let tokens = tokenize(source);
    println!("{tokens:?}");
}
