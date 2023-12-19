use std::iter::Peekable;
use crate::lexer::{Token, Tokens, tokenize};
use super::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
}

fn gettok(toks : &mut Peekable<Tokens>) -> ParserRes<Token> {
    Ok(toks.peek().ok_or(ParserError::EOF)?.clone())
}

fn parse_number(s : &String, _toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    Ok(Expr::Number(s.parse().map_err(|_| ParserError::InvalidNumber)?))
}

pub fn parse(toks : &mut Peekable<Tokens>) -> ParserRes<Expr> {
    match gettok(toks)? {
        Token::Number(s) => parse_number(&s, toks),
        _ => todo!(),
    }
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
