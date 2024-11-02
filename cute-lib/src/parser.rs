use crate::ast::Expression;
use crate::ast::{Add, Number};
use crate::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
struct Parser {
    tokens: Vec<Token>,
    loc: usize,
}

impl Parser {
    pub fn new(source: Vec<String>) -> Self {
        Self {
            tokens: parse_tokens(source),
            loc: 0,
        }
    }
    pub fn consume(&mut self, times_consume: usize) {
        for _ in 0..times_consume {
            self.tokens.remove(self.loc);
        }
    }
    pub fn peek(&self, offset: usize) -> Option<&Token> {
        if self.loc + offset >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.loc + offset])
    }
}

fn parse_expression(token: &Token) -> Option<Expression> {
    if token.kind() == &TokenKind::Number {
        return Some(Expression::Number(Number(
            token.value().parse::<u64>().unwrap(),
        )));
    }
    None
}

fn parse_expressions(parser: &mut Parser) -> Vec<Expression> {
    let mut exprs = Vec::new();

    loop {
        if parser.tokens.is_empty() {
            break;
        }
        let token = &parser.tokens[parser.loc];
        if parser.peek(1).is_some() && parser.peek(1).unwrap().kind() == &TokenKind::Plus {
            let lhs = parse_expression(&token).expect("Failed in parse_expression");
            let rhs = parse_expression(&parser.tokens[parser.loc + 2])
                .expect("Failed in parse_expression");
            exprs.push(Expression::Add(Box::new(Add::new(lhs, rhs))));
            parser.consume(3);
        } else if token.kind() == &TokenKind::Number {
            exprs.push(Expression::Number(Number(
                token.value().parse::<u64>().expect("Failed parsing number"),
            )));
            parser.consume(1);
        }
        parser.loc += 1;
    }
    exprs
}
fn parse_tokens(source: Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();
    for str in source {
        tokens.push(Token::new(&str));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_parse_tokens() {
        assert_eq!(parse_tokens(vec!["69".to_string()])[0], Token::new("69"));
    }
    #[test]
    fn check_parse_expressions() {
        let mut parser = Parser::new(vec!["69 ".to_string()]);
        assert_eq!(
            parse_expressions(&mut parser),
            vec![Expression::Number(Number(69))]
        );
        let mut parser = Parser::new(vec![
            "400 ".to_string(),
            "+ ".to_string(),
            "20 ".to_string(),
        ]);
        assert_eq!(
            parse_expressions(&mut parser),
            vec![Expression::Add(Box::new(Add::new(
                Expression::Number(Number(400)),
                Expression::Number(Number(20))
            )))]
        );
    }
}
