use std::process::Command;

pub fn compile_and_link(path: &str) {
    let mut nasm = Command::new("nasm");
    nasm.arg("-felf64");
    nasm.arg("-o");
    nasm.arg(format!("{path}.o"));
    nasm.arg(format!("{path}.asm"));
    nasm.status().unwrap();
    let mut ld = Command::new("ld");
    ld.arg("-o");
    ld.arg(format!("{path}"));
    ld.arg(format!("{path}.o"));
    ld.status().unwrap();
}

pub fn remove_file_extension(path: &str) -> String {
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

     path[..dot].to_string()
}

fn extract(accept: impl Fn(char) -> bool, string: &str) -> (&str, &str) {
    let extracted_end = string
        .char_indices()
        .find_map(
            |(index, char)| {
                if accept(char) {
                    None
                } else {
                    Some(index)
                }
            },
        )
        .unwrap_or(string.len());

    let remainder = &string[extracted_end..];
    let extracted = &string[..extracted_end];
    (remainder, extracted)
}

pub fn extract_whitespace(string: &str) -> (&str, &str) {
    extract(|char| char.is_whitespace(), string)
}

pub fn exrtact_until_char(string: &str, until: char) -> (&str, &str) {
    extract(|char| char != until, string)
}

pub fn extract_until_whitespace(string: &str) -> (&str, &str) {
    extract(|char| !char.is_whitespace(), string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_whitespace() {
        assert_eq!(extract_whitespace("    Hi"), ("Hi", "    "));
    }
    #[test]
    fn parse_until_whitespace() {
        assert_eq!(extract_until_whitespace("HelloTher e"), (" e", "HelloTher"));
    }
}
