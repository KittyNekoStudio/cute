use crate::expression::*;
use crate::lexer::{Token, TokenKind};
use crate::statement::{BlockStatement, ExpressionStatement, Statement, Statement::Expr};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Parser {
    tokens: Vec<Token>,
    loc: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum BindingPower {
    Default,
    //  Comma,
    Assignment,
    //  Logical,
    //  Relative,
    Additive,
    Multiplicitive,
    //  Unary,
    //  Call,
    //  Member,
    Primary,
}

type StatementHandler = fn(&mut Parser) -> Statement;
type NudHandler = fn(parser: &mut Parser) -> Expression;
type LedHandler =
    fn(parser: &mut Parser, left: &Expression, binding_power: &BindingPower) -> Expression;

type StatmentLookUp = HashMap<TokenKind, StatementHandler>;
type NudLookUp = HashMap<TokenKind, NudHandler>;
type LedLookUp = HashMap<TokenKind, LedHandler>;
type BpLookUp = HashMap<TokenKind, BindingPower>;

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, loc: 0 }
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
    fn expect(&mut self, expected_kind: TokenKind) -> Token {
        let token = self.current_token();
        let kind = token.kind;

        if kind != expected_kind {
            panic!("Expected TokenKind of: {:?}", expected_kind);
        }

        self.advance()
    }
}

fn create_lookups() -> (StatmentLookUp, NudLookUp, LedLookUp, BpLookUp) {
    (
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    )
}

fn led(
    bp_lookup: &mut BpLookUp,
    led_lookup: &mut LedLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    led_fn: LedHandler,
) {
    bp_lookup.insert(kind, binding_power);
    led_lookup.insert(kind, led_fn);
}

fn nud(
    bp_lookup: &mut BpLookUp,
    nud_lookup: &mut NudLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    nud_fn: NudHandler,
) {
    bp_lookup.insert(kind, binding_power);
    nud_lookup.insert(kind, nud_fn);
}

/*fn statement(
    mut bp_lookup: BpLookUp,
    mut stmt_lookup: StatmentLookUp,
    kind: TokenKind,
    binding_power: BindingPower,
    stmt_fn: StatementHandler,
) {
    bp_lookup.insert(kind, binding_power);
    stmt_lookup.insert(kind, stmt_fn);
}*/

fn create_token_lookups(
    _stmt_lookup: &mut StatmentLookUp,
    nud_lookup: &mut NudLookUp,
    led_lookup: &mut LedLookUp,
    bp_lookup: &mut BpLookUp,
) {
    // Literals and Symbols
    nud(
        bp_lookup,
        nud_lookup,
        TokenKind::Number,
        BindingPower::Primary,
        |parser| parse_primary_expression(parser),
    );

    nud(
        bp_lookup,
        nud_lookup,
        TokenKind::Symbol,
        BindingPower::Primary,
        |parser| parse_primary_expression(parser),
    );

    // Additive and Multiplicative
    led(
        bp_lookup,
        led_lookup,
        TokenKind::Plus,
        BindingPower::Additive,
        |parser, token, bp| parse_binary_expression(parser, token, bp),
    );

    led(
        bp_lookup,
        led_lookup,
        TokenKind::Minus,
        BindingPower::Additive,
        |parser, token, bp| parse_binary_expression(parser, token, bp),
    );

    led(
        bp_lookup,
        led_lookup,
        TokenKind::Mul,
        BindingPower::Multiplicitive,
        |parser, token, bp| parse_binary_expression(parser, token, bp),
    );

    led(
        bp_lookup,
        led_lookup,
        TokenKind::Div,
        BindingPower::Multiplicitive,
        |parser, token, bp| parse_binary_expression(parser, token, bp),
    );

    led(
        bp_lookup,
        led_lookup,
        TokenKind::Assignment,
        BindingPower::Assignment,
        |parser, token, bp| parse_binary_expression(parser, token, bp),
    );
}

fn parse_primary_expression(parser: &mut Parser) -> Expression {
    match parser.current_token_kind() {
        TokenKind::Number => {
            Expression::Number(Number(parser.advance().value.parse::<f64>().unwrap()))
        }
        TokenKind::Symbol => Expression::Symbol(Symbol(parser.advance().value)),
        TokenKind::Assignment => Expression::Symbol(Symbol(parser.advance().value)),
        _ => panic!(
            "Cannot create primary_expression from {:?}",
            parser.current_token_kind()
        ),
    }
}

fn parse_binary_expression(
    parser: &mut Parser,
    lhs: &Expression,
    binding_power: &BindingPower,
) -> Expression {
    let op = parser.advance();
    let rhs = parse_expression(parser, &binding_power);
    Expression::Binary(Box::new(BinaryExpression::new(lhs.to_owned(), op, rhs)))
}

fn parse_expression(parser: &mut Parser, binding_power: &BindingPower) -> Expression {
    let (mut stmt_lookup, mut nud_lookup, mut led_lookup, mut bp_lookup) = create_lookups();
    create_token_lookups(
        &mut stmt_lookup,
        &mut nud_lookup,
        &mut led_lookup,
        &mut bp_lookup,
    );

    // First parse the NUD
    let mut token_kind = parser.current_token_kind();
    if !nud_lookup.contains_key(&token_kind) {
        panic!("Nud Handler Expected For TokenKind {token_kind:?}");
    }

    let nud_fn = nud_lookup[&token_kind];
    let mut lhs = nud_fn(parser);

    while let Some(power) = bp_lookup.get(&parser.current_token_kind()) {
        // While we have a LED and the currend binding_power is < binding_power of the current token
        // Continue parsing left hand side
        if power > binding_power {
            token_kind = parser.current_token_kind();
            if !led_lookup.contains_key(&token_kind) {
                panic!("Led Handler Expected For TokenKind {token_kind:?}");
            }

            let led_fn = led_lookup[&token_kind];

            lhs = led_fn(parser, &lhs, binding_power);
        }
    }
    lhs
}

pub fn parse(tokens: Vec<Token>) -> BlockStatement {
    let mut body = Vec::new();
    let mut parser = Parser::new(tokens);

    while parser.has_tokens() {
        body.push(parse_statement(&mut parser));
    }

    BlockStatement { body }
}

fn parse_statement(parser: &mut Parser) -> Statement {
    let (mut stmt_lookup, mut nud_lookup, mut led_lookup, mut bp_lookup) = create_lookups();
    create_token_lookups(
        &mut stmt_lookup,
        &mut nud_lookup,
        &mut led_lookup,
        &mut bp_lookup,
    );

    let stmt_fn = stmt_lookup.get(&parser.current_token_kind());
    match stmt_fn {
        Some(stmt_fn) => stmt_fn(parser),
        None => {
            let expression = parse_expression(parser, &BindingPower::Default);
            parser.expect(TokenKind::EOL);
            Expr(ExpressionStatement::new(expression))
        }
    }
}
