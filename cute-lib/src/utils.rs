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

pub fn extract_whitespace(string: &str) -> (&str, &str) {
    extract(|char| char.is_whitespace(), string)
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
