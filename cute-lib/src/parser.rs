use crate::expression::Expression;
use crate::expression::Expression::{Number, Symbol};
use crate::lexer::{Token, TokenKind};
use crate::statement::{BlockStatement, Statement};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Parser {
    tokens: Vec<Token>,
    loc: usize,
}

#[derive(Debug, PartialEq, Clone)]
enum BindingPower {
    Default,
    Comma,
    Assignment,
    Logical,
    Relative,
    Multiplicitive,
    Unary,
    Call,
    Member,
    Primary,
}

type StatementHandler = fn(&mut Parser) -> Statement;
type NudHandler = fn(parser: &mut Parser) -> Expression;
type LedHandler = for<'a> fn(
    parser: &mut Parser,
    left: Expression<'a>,
    binding_power: BindingPower,
) -> Expression<'a>;

type StatmentLookUp = HashMap<TokenKind, StatementHandler>;
type NudLookUp = HashMap<TokenKind, NudHandler>;
type LedLookUp = HashMap<TokenKind, LedHandler>;
type BpLookUp = HashMap<TokenKind, BindingPower>;

fn create_lookups() -> (StatmentLookUp, NudLookUp, LedLookUp, BpLookUp) {
    (
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    )
}

fn led(
    mut bp_lookup: BpLookUp,
    mut led_lookup: LedLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    led_fn: LedHandler,
) {
    bp_lookup.insert(kind, binding_power);
    led_lookup.insert(kind, led_fn);
}

fn nud(
    mut bp_lookup: BpLookUp,
    mut nud_lookup: NudLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    nud_fn: NudHandler,
) {
    bp_lookup.insert(kind, binding_power);
    nud_lookup.insert(kind, nud_fn);
}

fn statement(
    mut bp_lookup: BpLookUp,
    mut stmt_lookup: StatmentLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    nud_fn: StatementHandler,
) {
    bp_lookup.insert(kind, binding_power);
    stmt_lookup.insert(kind, nud_fn);
}

fn create_token_lookups(
    stmt_lookup: StatmentLookUp,
    nud_lookup: NudLookUp,
    led_lookup: LedLookUp,
    bp_lookup: BpLookUp,
) -> (StatmentLookUp, NudLookUp, LedLookUp, BpLookUp) {
    // Literals and Symbols

    nud(
        bp_lookup.clone(),
        nud_lookup.clone(),
        TokenKind::Number,
        BindingPower::Primary,
        |parser| parse_primary_expression(parser),
    );

    nud(
        bp_lookup.clone(),
        nud_lookup.clone(),
        TokenKind::Identifier,
        BindingPower::Primary,
        |parser| parse_primary_expression(parser),
    );

    (stmt_lookup, nud_lookup, led_lookup, bp_lookup)
}

fn parse_primary_expression(parser: &mut Parser) -> Expression {
    match parser.current_token_kind() {
        TokenKind::Number => Number(parser.advance().value.parse::<f64>().unwrap()),
        TokenKind::Identifier => Symbol(parser.advance().value),
        _ => panic!(
            "Cannot create primary_expression from {:?}",
            parser.current_token_kind()
        ),
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let (stmt, nud, led, bp) = create_lookups();
        let (stmt, nud, led, bp) = create_token_lookups(stmt, nud, led, bp);

        Self { tokens, loc: 0 }
    }
    pub fn parse(tokens: Vec<Token>) -> BlockStatement {
        let mut body = Vec::new();
        let mut parser = Parser::new(tokens);

        while parser.has_tokens() {
            body.push(parse_statement(&parser));
        }

        return BlockStatement { body };
    }
    fn current_token(&self) -> Token {
        self.tokens[self.loc].clone()
    }
    fn current_token_kind(&self) -> TokenKind {
        self.current_token().kind
    }
    fn advance(&mut self) -> Token {
        let token = self.current_token();
        self.loc += 1;
        token
    }
    fn has_tokens(&self) -> bool {
        self.loc < self.tokens.len() && self.current_token_kind() != TokenKind::EOF
    }
}

fn parse_statement(parser: &Parser) -> Statement {
    todo!();
}
