use std::{str::Chars, iter::Peekable};
use super::error::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    // Keywords
    Function,
    Return,

    LParen, RParen, // ( )
    LBrack, RBrack, // { }
}

fn skip_whitespace(chars : &mut Peekable<Chars>) -> LexerRes<char> {
    while let Some(c) = chars.peek() {
        if !c.is_whitespace() {
            return Ok(*c)
        }
        chars.next();
    }
    Err(LexerError::EOF)
}

fn collect_while(chars : &mut Peekable<Chars>, f : impl Fn(&char) -> bool) -> String {
    let mut s = String::new();
    while let Some(c) = chars.peek() {
        if !f(c) {
            break;
        }
        s.push(*c);
        chars.next();
    }
    return s;
}

fn get_number(chars : &mut Peekable<Chars>) -> LexerRes<Token> {
    // TODO: Different bases and '_'
    Ok(Token::Number(collect_while(chars, |c| c.is_digit(10))))
}

fn get_ident(chars : &mut Peekable<Chars>) -> LexerRes<Token> {
    let ident = collect_while(chars, |c| c.is_alphanumeric() || *c == '_');
    Ok(match &*ident {
        "fn" => Token::Function,
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    })
}

pub fn gettok(chars : &mut Peekable<Chars>) -> LexerRes<Token> {
    let c = skip_whitespace(chars)?;

    if c.is_digit(10) {
        return get_number(chars)
    } else if c.is_alphabetic() {
        return get_ident(chars)
    } else {
        chars.next(); // TODO: Should this happen here? It might depend on _ branch
        match c {
            '(' => Ok(Token::LParen), ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrack), '}' => Ok(Token::RBrack),
            _ => todo!(),
        }
    }
}

pub fn gettok_str(s : &str) -> LexerRes<Token> {
    gettok(&mut s.chars().peekable())
}

pub fn gettok_string(s : &String) -> LexerRes<Token> {
    gettok_str(&*s)
}

#[test]
fn test_get_number() {
    assert_eq!(gettok_str("0"), Ok(Token::Number("0".to_string())));
    assert_eq!(gettok_str(" 1"), Ok(Token::Number("1".to_string())));
    assert_eq!(gettok_str("2  3"), Ok(Token::Number("2".to_string())));
}

#[test]
fn test_get_identifier() {
    assert_eq!(gettok_str("g"), Ok(Token::Identifier("g".to_string())));
    assert_eq!(gettok_str("galileo"), Ok(Token::Identifier("galileo".to_string())));
    assert_eq!(gettok_str("gali leo"), Ok(Token::Identifier("gali".to_string())));
    assert_eq!(gettok_str("g4l1l30"), Ok(Token::Identifier("g4l1l30".to_string())));
    assert_eq!(gettok_str("gali_leo"), Ok(Token::Identifier("gali_leo".to_string())));

    assert_eq!(gettok_str("fn"), Ok(Token::Function));
    assert_eq!(gettok_str("return"), Ok(Token::Return));
}
