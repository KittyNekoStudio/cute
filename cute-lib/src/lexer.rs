use crate::statement::GenerateAsm;
use crate::utils::{extract_until_whitespace, extract_whitespace};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenKind {
    // Types
    Number,
    Symbol,

    // Operations
    Plus,
    Minus,
    Mul,
    Div,
    Binding,
    Assignment,

    // Reserved keywords
    Let,
    MutLet,

    EOL,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: Vec<String>,
    loc: (i32, i32),
}

#[derive(Debug, PartialEq)]
struct KeywordMap {
    hashmap: HashMap<String, TokenKind>,
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
    fn previous(&mut self) -> Option<&Token> {
        if self.tokens.last().is_none() {
            None
        } else {
            Some(&self.tokens.last().unwrap())
        }
    }
}

impl GenerateAsm for Token {
    fn generate(&self, file: &mut File) {
        if self.kind == TokenKind::Plus {
            write!(file, "    mov  rdi, {}\n", 3).unwrap();
        }
    }
}

pub fn tokenize(source: Vec<String>) -> Vec<Token> {
    let mut lexer = Lexer::new(source.clone());
    let keywords = KeywordMap::create();
    for mut str in source {
        while !str.is_empty() {
            let previous = lexer.previous();
            str = extract_whitespace(&str).0.to_string();
            // I don't want to have semicolons so this strips them or panics if the file contains semicolons
            if str == ";" {
                str = "".to_string();
                continue;
            }
            let (string, value) = extract_until_whitespace(&str);
            let value = value.strip_prefix(";").unwrap_or(value);
            let value = value.strip_suffix(";").unwrap_or(value);
            if value.contains(";") {
                panic!("Semi-colon is not allowed inside values");
            }
            if value.parse::<f64>().is_ok() {
                lexer.push_token(Token::new(TokenKind::Number, value));
            } else if value == "+" {
                lexer.push_token(Token::new(TokenKind::Plus, value));
            } else if value.starts_with("\"") && value.ends_with("\"") {
                let value = value.strip_prefix("\"").unwrap_or(value);
                let value = value.strip_suffix("\"").unwrap_or(value);
                lexer.push_token(Token::new(TokenKind::Symbol, value));
            } else if value == "-" {
                lexer.push_token(Token::new(TokenKind::Minus, value));
            } else if value == "*" {
                lexer.push_token(Token::new(TokenKind::Mul, value));
            } else if value == "/" {
                lexer.push_token(Token::new(TokenKind::Div, value));
            } else if keywords.get(value).is_some() {
                if previous == None {
                    lexer.push_token(Token::new(
                        *keywords
                            .get(value)
                            .expect("failed in pushing keyword from map in lexer"),
                        value,
                    ));
                } else {
                    match value {
                        "let" => {
                            assert!(
                                previous.unwrap().kind != TokenKind::Let
                                    && previous.unwrap().kind != TokenKind::MutLet,
                                "Variable name is a keyword"
                            );
                            lexer.push_token(Token::new(TokenKind::Let, value));
                        }
                        "let!" => {
                            assert!(
                                previous.unwrap().kind != TokenKind::Let
                                    && previous.unwrap().kind != TokenKind::MutLet,
                                "Variable name is a keyword"
                            );
                            lexer.push_token(Token::new(TokenKind::MutLet, value));
                        }
                        _ => (),
                    }
                }
            } else if previous.is_some()
                && keywords
                    .get(
                        &previous
                            .expect("Failed Checking If Previous Token Is A Keyword")
                            .value,
                    )
                    .is_some()
            {
                if previous.unwrap().kind == TokenKind::Let
                    || previous.unwrap().kind == TokenKind::MutLet
                {
                    lexer.push_token(Token::new(TokenKind::Binding, value));
                }
            } else if value == "=" {
                lexer.push_token(Token::new(TokenKind::Assignment, value));
            }
            // TODO! fix comment handling
            // comments don't return token before // when there is no space on both sides
            // eg. 69//420
            // should return 69 but returns nothing
            else if value.contains("//") {
                if value.ends_with("//") {
                    let value = &value[..value.len() - 2];
                    str = value.to_string();
                    continue;
                } else {
                    str = "".to_string();
                    continue;
                }
            }
            str = string.to_string();
            if str == "" {
                lexer.push_token(Token::new(TokenKind::EOL, "EOL"));
            }
            lexer.loc = (lexer.loc.0, lexer.loc.1 + 1);
        }
        lexer.loc = (lexer.loc.0 + 1, 0);
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

impl KeywordMap {
    fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
    pub fn create() -> Self {
        let mut keyword_map = Self::new();
        keyword_map
            .hashmap
            .insert("let".to_string(), TokenKind::Let);
        keyword_map
            .hashmap
            .insert("let!".to_string(), TokenKind::MutLet);
        keyword_map
    }
    pub fn get(&self, key: &str) -> Option<&TokenKind> {
        self.hashmap.get(key)
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
                    kind: TokenKind::EOL,
                    value: "EOL".to_string()
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
                    kind: TokenKind::EOL,
                    value: "EOL".to_string()
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
                    kind: TokenKind::Minus,
                    value: "-".to_string()
                },
                Token {
                    kind: TokenKind::EOL,
                    value: "EOL".to_string()
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
                    kind: TokenKind::Mul,
                    value: "*".to_string()
                },
                Token {
                    kind: TokenKind::EOL,
                    value: "EOL".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }
    /*#[test]
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
                    kind: TokenKind::Minus,
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
                    kind: TokenKind::EOL,
                    value: "EOL".to_string()
                },
                Token {
                    kind: TokenKind::EOF,
                    value: "EOF".to_string()
                }
            ]
        );
    }*/
    #[test]
    fn check_keyword_hashmap() {
        let keyword_map = KeywordMap::create();
        assert_eq!(keyword_map.get("let").unwrap(), &TokenKind::Let);
        assert_eq!(keyword_map.get("let!").unwrap(), &TokenKind::MutLet);
    }
}
