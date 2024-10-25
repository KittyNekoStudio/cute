use cute_lib::*;
fn main() {
    let mut buffer = Buffer::new();
    write_to_file("foo.cute", &mut buffer);
}
