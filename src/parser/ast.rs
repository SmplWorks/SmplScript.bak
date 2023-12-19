use std::iter::Peekable;
use crate::lexer::{Token, Tokens, tokenize};
use super::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Block(Vec<Expr>),
}

fn peektok(toks : &mut Peekable<Tokens>) -> ParserRes<Token> {
    Ok(toks.peek().ok_or(ParserError::EOF)?.clone())
}

fn nexttok(toks : &mut Peekable<Tokens>) -> ParserRes<Token> {
    Ok(toks.next().ok_or(ParserError::EOF)?.clone())
}

fn collect_while(toks : &mut Peekable<Tokens>, f : impl Fn(&Token) -> bool) -> ParserRes<Vec<Expr>> {
    let mut exprs = vec![];
    loop {
        let t = nexttok(toks)?;
        if f(&t) {
            break;
        }

        exprs.push(parse_tok(t, toks)?);
    }
    return Ok(exprs);
}

fn parse_number(s : &String, _toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    Ok(Expr::Number(s.parse().map_err(|_| ParserError::InvalidNumber)?))
}

fn parse_block(toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    Ok(Expr::Block(collect_while(toks, |t| *t == Token::RBrack)?))
}

fn parse_tok(t : Token, toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    match t {
        Token::Number(s) => parse_number(&s, toks),
        Token::LBrack => parse_block(toks),
        _ => todo!("{:?}", peektok(toks)),
    }
}

pub fn parse(toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    parse_tok(nexttok(toks)?, toks)
}

pub fn parse_str(s : &str) -> ParserRes<Expr> {
    parse(&mut tokenize(s.chars()).peekable())
}

pub fn parse_string(s : &String) -> ParserRes<Expr> {
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
}
