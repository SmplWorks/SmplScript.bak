use std::iter::Peekable;
use crate::lexer::{Token, Tokens, tokenize};
use crate::utils::{SError, SRes};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Block(Vec<Expr>),
    Function{
        name : String,
        params : Vec<String>,
        body : Box<Expr>,
    },
    Return(Box<Expr>),
    VarRef(String),
    BinaryOp{
        op: String,
        lhs : Box<Expr>,
        rhs : Box<Expr>,
    },
}

fn peektok(toks : &mut Peekable<Tokens>) -> SRes<Token> {
    Ok(toks.peek().ok_or(SError::LexerEOF)?.clone())
}

fn nexttok(toks : &mut Peekable<Tokens>) -> SRes<Token> {
    Ok(toks.next().ok_or(SError::LexerEOF)?.clone())
}

fn collect_while(toks : &mut Peekable<Tokens>, f : impl Fn(&Token) -> bool) -> SRes<Vec<Expr>> {
    let mut exprs = vec![];
    loop {
        let t = nexttok(toks)?;
        if !f(&t) {
            break;
        }

        exprs.push(parse_tok(t, toks)?);
    }
    return Ok(exprs);
}

fn parse_number(s : &String, _toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    Ok(Expr::Number(s.parse().map_err(|_| SError::ParserInvalidNumber)?))
}

fn parse_block(toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    Ok(Expr::Block(collect_while(toks, |t| *t != Token::RBrack)?))
}

fn parse_paren(toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    let expr = parse(toks)?;
    match nexttok(toks) {
        Ok(Token::RParen) => Ok(expr),
        Ok(_) | Err(SError::LexerEOF) => Err(SError::ParserExpectedClosingParen),
        Err(err) => Err(err),
    }
}

fn parse_params(toks : &mut Peekable<Tokens>) -> SRes<Vec<String>> {
    if nexttok(toks)? != Token::LParen { // Check for '('
        return Err(SError::ParserInvalidFunctionNoLParen)
    }

    let mut params = vec![]; 
    let mut first = true;
    let mut expect_identifier = true;
    loop {
        match nexttok(toks)? {
            Token::Identifier(s) => if expect_identifier {
                expect_identifier = false;
                params.push(s);
            } else { return Err(SError::ParserInvalidFunctionMissingComma) },
            Token::Comma => if !expect_identifier {
                expect_identifier = true;
            } else { return Err(SError::ParserInvalidFunctionExtraComma) },
            Token::RParen => if !expect_identifier || first {
                break
            } else { return Err(SError::ParserInvalidFunctionExpectedParam) },
            _ => return Err(SError::ParserInvalidFunctionInvalidToken)
        }

        first = false;
    }
    return Ok(params);
}

fn parse_function(toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    let Token::Identifier(name) = nexttok(toks)? else { return Err(SError::ParserInvalidFunctionNoName) };
    let params = parse_params(toks)?;
    let body = Box::new(parse(toks)?);

    Ok(Expr::Function { name, params, body })
}

fn parse_return(toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    return Ok(Expr::Return(Box::new(parse(toks)?)))
}

fn parse_identifier(s : &String, _toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    Ok(Expr::VarRef(s.clone()))
}

fn parse_primary(t : Token, toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    match t {
        Token::Number(s) => parse_number(&s, toks),
        Token::LBrack => parse_block(toks),
        Token::LParen => parse_paren(toks),
        Token::Function => parse_function(toks),
        Token::Return => parse_return(toks),
        Token::Identifier(s) => parse_identifier(&s, toks),
        _ => todo!("{:?}", t),
    }
}

fn parse_binop_rhs(expr_prec : i32, mut lhs : Expr, toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    loop {
        let tok_prec = peektok(toks).map_or(-1, |t| t.get_precedence());
        if tok_prec < expr_prec {
            return Ok(lhs)
        }

        let op = nexttok(toks)?.to_string();

        match nexttok(toks) {
            Ok(t) => {
                let mut rhs = parse_primary(t, toks)?;
                let next_prec = peektok(toks).map_or(-1, |t| t.get_precedence());
                if tok_prec < next_prec {
                    rhs = parse_binop_rhs(tok_prec+1, rhs, toks)?;
                }

                lhs = Expr::BinaryOp{ op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
            },
            Err(SError::LexerEOF) => return Ok(lhs),
            Err(err) => return Err(err),
        }
    }
}

fn parse_tok(t : Token, toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    let lhs = parse_primary(t, toks)?;
    if let Err(SError::LexerEOF) = peektok(toks) {
        return Ok(lhs);
    }

    parse_binop_rhs(0, lhs, toks)
}

pub fn parse(toks : &mut Peekable<Tokens>) -> SRes<Expr> {
    parse_tok(nexttok(toks)?, toks)
}

pub fn parse_str(s : &str) -> SRes<Expr> {
    parse(&mut tokenize(s.chars()).peekable())
}

pub fn parse_string(s : &String) -> SRes<Expr> {
    parse_str(&*s)
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_str("0"), Ok(Expr::Number(0)));
}

#[test]
fn test_parse_block() {
    assert_eq!(parse_str("{}"), Ok(Expr::Block(vec![])));
    assert_eq!(parse_str("{0}"), Ok(Expr::Block(vec![Expr::Number(0)])));
    assert_eq!(parse_str("{{0}}"), Ok(Expr::Block(vec![Expr::Block(vec![Expr::Number(0)])])));
    assert_eq!(parse_str("{{0 1}}"), Ok(Expr::Block(vec![Expr::Block(vec![Expr::Number(0), Expr::Number(1)])])));
    assert_eq!(parse_str("{"), Err(SError::LexerEOF));
}

#[test]
fn test_parse_paren() {
    assert_eq!(parse_str("(0)"), Ok(Expr::Number(0)));
    assert_eq!(parse_str("((0))"), Ok(Expr::Number(0)));
    assert_eq!(parse_str("("), Err(SError::LexerEOF));
    assert_eq!(parse_str("(0"), Err(SError::ParserExpectedClosingParen));
}

#[test]
fn test_parse_function() {
    assert_eq!(parse_str("fn zero() 0"), Ok(Expr::Function{name: "zero".to_string(), params: vec![], body: Box::new(Expr::Number(0))}));
    assert_eq!(parse_str("fn oneParam(x) {}"), Ok(Expr::Function{name: "oneParam".to_string(), params: vec!["x".to_string()], body: Box::new(Expr::Block(vec![]))}));
    assert_eq!(parse_str("fn twoParams(x, y) {}"), Ok(Expr::Function{name: "twoParams".to_string(), params: vec!["x".to_string(), "y".to_string()], body: Box::new(Expr::Block(vec![]))}));
    assert_eq!(parse_str("fn () {}"), Err(SError::ParserInvalidFunctionNoName));
    assert_eq!(parse_str("fn main {}"), Err(SError::ParserInvalidFunctionNoLParen));
    assert_eq!(parse_str("fn main (x y) {}"), Err(SError::ParserInvalidFunctionMissingComma));
    assert_eq!(parse_str("fn main (,) {}"), Err(SError::ParserInvalidFunctionExtraComma));
    assert_eq!(parse_str("fn main (,x) {}"), Err(SError::ParserInvalidFunctionExtraComma));
    assert_eq!(parse_str("fn main (x,) {}"), Err(SError::ParserInvalidFunctionExpectedParam));
    assert_eq!(parse_str("fn main (fn x) {}"), Err(SError::ParserInvalidFunctionInvalidToken));
}

#[test]
fn test_parse_return() {
    assert_eq!(parse_str("return 0"), Ok(Expr::Return(Box::new(Expr::Number(0)))));
    assert_eq!(parse_str("return"), Err(SError::LexerEOF)); // TODO: Allow return with nothing
}

#[test]
fn test_parse_varref() {
    assert_eq!(parse_str("x"), Ok(Expr::VarRef("x".to_string())));
}

#[test]
fn test_parse_binaryop() {
    assert_eq!(parse_str("0 + 1"), Ok(Expr::BinaryOp { op: "+".to_string(),  lhs: Box::new(Expr::Number(0)), rhs: Box::new(Expr::Number(1)) }));
    assert_eq!(parse_str("0 + 1 - 2"), Ok(Expr::BinaryOp { op: "-".to_string(), lhs: Box::new(Expr::BinaryOp { op: "+".to_string(),  lhs: Box::new(Expr::Number(0)), rhs: Box::new(Expr::Number(1)) }), rhs: Box::new(Expr::Number(2))}));
    assert_eq!(parse_str("0 + 1 * 2"), Ok(Expr::BinaryOp { op: "+".to_string(),  lhs: Box::new(Expr::Number(0)), rhs: Box::new(Expr::BinaryOp { op: "*".to_string(), lhs: Box::new(Expr::Number(1)), rhs: Box::new(Expr::Number(2)) }) }));

    assert_eq!(parse_str("x = 0"), Ok(Expr::BinaryOp { op: "=".to_string(),  lhs: Box::new(Expr::VarRef("x".to_string())), rhs: Box::new(Expr::Number(0)) }));
    assert_eq!(parse_str("x = y"), Ok(Expr::BinaryOp { op: "=".to_string(),  lhs: Box::new(Expr::VarRef("x".to_string())), rhs: Box::new(Expr::VarRef("y".to_string())) }));
    assert_eq!(parse_str("0 = x"), Ok(Expr::BinaryOp { op: "=".to_string(),  lhs: Box::new(Expr::Number(0)), rhs: Box::new(Expr::VarRef("x".to_string())) }));
}
