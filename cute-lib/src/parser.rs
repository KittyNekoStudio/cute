use crate::ast::Expression;
use crate::ast::{Add, Number};
use crate::lexer::{parse_tokens, Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
/// Parser
/// Holds a vector of tokens and keeps track of where in the vector it's at.
pub struct Parser {
    tokens: Vec<Token>,
    loc: usize,
}

impl Parser {
    /// Takes a vector of strings and calls parse_tokens.
    pub fn new(source: Vec<String>) -> Self {
        Self {
            tokens: parse_tokens(source),
            loc: 0,
        }
    }
    /// Removes from loc n number of times.
    pub fn consume(&mut self, index: usize) -> Token {
        self.loc += index;
        // TODO! remove clone
        self.tokens[index - 1].clone()
    }
    /// Looks ahead of loc + an offset.
    pub fn peek(&self, offset: usize) -> Option<&Token> {
        if self.loc + offset >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.loc + offset])
    }
}

/// Parses a single expression.
/// Used in parse_expressions to parse a token further in the vector of tokens.
fn parse_expression(token: &Token) -> Option<Expression> {
    if token.kind() == &TokenKind::Number {
        return Some(Expression::Number(Number(
            token.value().parse::<u64>().unwrap(),
        )));
    }
    None
}

/// Loops through all the tokens the parser holds.
/// Returns a vector of expressions.
pub fn parse_expressions(parser: &mut Parser) -> Vec<Expression> {
    let mut exprs = Vec::new();

    loop {
        if parser.loc >= parser.tokens.len() {
            break;
        }

        // TODO! we enter an infinite loop when we add multiple times. eg.
        // 3 + 2 + 10
        let token = &parser.tokens[parser.loc];
        if parser.peek(1).is_some() && parser.peek(1).unwrap().kind() == &TokenKind::Plus {
            let lhs = parse_expression(&token).expect("Failed in parse_expression");
            let rhs = parse_expression(&parser.consume(3)).expect("Failed in parse_expression");
            exprs.push(Expression::Add(Box::new(Add::new(lhs, rhs))));
        } else if token.kind() == &TokenKind::Number {
            exprs.push(Expression::Number(Number(
                token.value().parse::<u64>().expect("Failed parsing number"),
            )));
            parser.consume(1);
        } else if token.kind() == &TokenKind::EOL {
            parser.consume(1);
        } else if token.kind() == &TokenKind::EOF {
            parser.consume(1);
        }
        //println!("{}", parser.loc);
    }
    exprs
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
        let mut parser = Parser::new(vec!["69".to_string()]);
        assert_eq!(
            parse_expressions(&mut parser),
            vec![Expression::Number(Number(69))]
        );
        let mut parser = Parser::new(vec!["400 + 20".to_string()]);
        assert_eq!(
            parse_expressions(&mut parser),
            vec![Expression::Add(Box::new(Add::new(
                Expression::Number(Number(400)),
                Expression::Number(Number(20))
            )))]
        );
    }
}
