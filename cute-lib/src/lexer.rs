#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(str: &str) -> Self {
        let (value, _) = parse_token(str);
        let kind = parse_token_kind(value).expect("TokenKind not recognized");
        Self {
            kind,
            value: value.to_string(),
        }
    }
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Number,
    Plus,
}

fn parse_token(str: &str) -> (&str, &str) {
    let extracted_index = str
        .char_indices()
        .find_map(|(index, char)| {
            if char.is_whitespace() {
                return Some(index);
            } else {
                None
            }
        })
        .unwrap_or_else(|| 0);

    let value = if str.len() == 1 {
        str
    } else {
        &str[..extracted_index]
    };

    let remainder = if str.len() == 1 {
        ""
    } else {
        &str[extracted_index..]
    };

    (value, remainder)
}

fn parse_token_kind(str: &str) -> Option<TokenKind> {
    if str.chars().all(|char| char::is_ascii_digit(&char)) {
        return Some(TokenKind::Number);
    } else if str == "+" {
        return Some(TokenKind::Plus);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_token_number() {
        assert_eq!(
            Token::new("1"),
            Token {
                kind: TokenKind::Number,
                value: "1".to_string()
            }
        );
    }
    #[test]
    fn lex_token_plus() {
        assert_eq!(
            Token::new("+"),
            Token {
                kind: TokenKind::Plus,
                value: "+".to_string()
            }
        );
    }
    #[test]
    fn check_parse_token() {
        assert_eq!(parse_token("+"), ("+", ""));
    }
    #[test]
    fn check_parse_token_kind() {
        assert_eq!(parse_token_kind("69").unwrap(), TokenKind::Number);
        assert_eq!(parse_token_kind("+").unwrap(), TokenKind::Plus);
    }
}
