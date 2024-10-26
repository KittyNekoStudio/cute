use crate::utils::{extract_until_whitespace, extract_whitespace};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number,
    Plus,
    Dash,
    Star,
    Let,
    Binding,
    Assignment,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: Vec<String>,
    loc: (i32, i32),
}

impl Lexer {
    pub fn new(source: Vec<String>) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            loc: (0, 0),
        }
    }
    fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn previous(&mut self) -> &TokenKind {
        &self.tokens.last().unwrap().kind
    }
}

pub fn tokenize(source: Vec<String>) -> Vec<Token> {
    let mut lexer = Lexer::new(source.clone());
    for mut str in source {
        while !str.is_empty() {
            str = extract_whitespace(&str).0.to_string();
            let (string, value) = extract_until_whitespace(&str);
            if value.parse::<i32>().is_ok() {
                lexer.push_token(Token::new(TokenKind::Number, value));
            } else if value == "+" {
                lexer.push_token(Token::new(TokenKind::Plus, value));
            } else if value == "-" {
                lexer.push_token(Token::new(TokenKind::Dash, value));
            } else if value == "*" {
                lexer.push_token(Token::new(TokenKind::Star, value));
            } else if value == "let" {
                lexer.push_token(Token::new(TokenKind::Let, value));
            } else if lexer.previous() == &TokenKind::Let {
                lexer.push_token(Token::new(TokenKind::Binding, value));
            } else if value == "=" {
                lexer.push_token(Token::new(TokenKind::Assignment, value));
            }
            str = string.to_string();
            lexer.loc = (lexer.loc.0, lexer.loc.1 + 1);
        }
        lexer.loc = (lexer.loc.0, lexer.loc.1 + 1);
    }
    lexer.push_token(Token::new(TokenKind::EOF, "EOF"));
    lexer.tokens
}

impl Token {
    pub fn new(kind: TokenKind, value: &str) -> Self {
        Self {
            kind,
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_token() {
        assert_eq!(
            Token::new(TokenKind::Number, "9"),
            Token {
                kind: TokenKind::Number,
                value: "9".to_string()
            }
        );
    }
    #[test]
    fn create_empty_lexer() {
        assert_eq!(
            Lexer::new(vec![]),
            Lexer {
                tokens: vec![],
                source: vec![],
                loc: (0, 0)
            }
        );
    }
    #[test]
    fn create_lexer_with_source() {
        assert_eq!(
            Lexer::new(vec!["9".to_string(), "30 + 2".to_string()]),
            Lexer {
                tokens: vec![],
                source: vec!["9".to_string(), "30 + 2".to_string()],
                loc: (0, 0)
            }
        );
    }

    #[test]
    fn tokenize_number() {
        let source = vec!["3209".to_string()];
        let token = tokenize(source);
        assert_eq!(
            token,
            vec![
                Token {
                    kind: TokenKind::Number,
                    value: "3209".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
    #[test]
    fn tokenize_plus() {
        let source = vec!["+".to_string()];
        let token = tokenize(source);
        assert_eq!(
            token,
            vec![
                Token {
                    kind: TokenKind::Plus,
                    value: "+".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
    #[test]
    fn tokenize_dash() {
        let source = vec!["-".to_string()];
        let token = tokenize(source);
        assert_eq!(
            token,
            vec![
                Token {
                    kind: TokenKind::Dash,
                    value: "-".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
    #[test]
    fn tokenize_star() {
        let source = vec!["*".to_string()];
        let token = tokenize(source);
        assert_eq!(
            token,
            vec![
                Token {
                    kind: TokenKind::Star,
                    value: "*".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
    #[test]
    fn tokenize_all() {
        let source = vec![
            "9".to_string(),
            "30 + 2".to_string(),
            "- 3".to_string(),
            "4 * 3".to_string(),
        ];
        let tokens = tokenize(source);
        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: TokenKind::Number,
                    value: "9".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    value: "30".to_string()
                },
                Token {
                    kind: TokenKind::Plus,
                    value: "+".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    value: "2".to_string()
                },
                Token {
                    kind: TokenKind::Dash,
                    value: "-".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    value: "3".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    value: "4".to_string()
                },
                Token {
                    kind: TokenKind::Star,
                    value: "*".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    value: "3".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
}
