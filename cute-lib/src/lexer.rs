#[derive(Debug, PartialEq, Clone)]
/// Token
/// Holds a TokenKind and a value as a string.
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    /// Takes in a &str and uses it to parse the token kind,
    /// as well as the value.
    pub fn new(str: &str) -> Self {
        let (value, _) = parse_token(str);
        let kind = parse_token_kind(value).expect("TokenKind not recognized");
        Self {
            kind,
            value: value.to_string(),
        }
    }
    /// Returns a reference to the TokenKind.
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
    /// Returns a reference to the value.
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, PartialEq, Clone)]
/// TokenKind
/// All the variants that a token can be.
pub enum TokenKind {
    Number,
    Plus,
}

/// Parses the value for the token.
/// Returns the value and the remainder of the &str.
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

/// Parses the token kind from a &str.
fn parse_token_kind(str: &str) -> Option<TokenKind> {
    if str.chars().all(|char| char::is_ascii_digit(&char)) {
        return Some(TokenKind::Number);
    } else if str == "+" {
        return Some(TokenKind::Plus);
    }
    None
}

/// Parses each String in a vector.
/// Returns a vector of tokens.
pub fn parse_tokens(source: Vec<String>) -> Vec<Token> {
    source.iter().map(|str| Token::new(&str)).collect()
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
