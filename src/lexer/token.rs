use std::{str::Chars, iter::Peekable, fmt::Display};
use super::error::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    // Keywords
    Function, // fn
    Return, // return
    Let, // let

    LParen, RParen, // ( )
    LBrack, RBrack, // { }
    Comma, SemiColon, // , ;
    Assign, // =
}

impl Token {
    pub fn is_binary_operator(&self) -> bool {
        match self {
            Self::Assign => true,
            _ => false,
        }
    }

    pub fn get_precedence(&self) -> i32 {
        match self {
            Self::Assign => 2,
            _ => -1,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(s) => write!(f, "{s}"),
            Self::Identifier(s) => write!(f, "{s}"),
            Self::Function => write!(f, "fn"),
            Self::Return => write!(f, "return"),
            Self::Let => write!(f, "let"),
            Self::LParen => write!(f, "("), Token::RParen => write!(f, ")"),
            Self::LBrack => write!(f, "{}", '{'), Token::RBrack => write!(f, "{}", '}'),
            Self::Comma => write!(f, ","), Token::SemiColon => write!(f, ";"),
            Self::Assign => write!(f, "="),
        }
    }
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
        "let" => Token::Let,
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
            ',' => Ok(Token::Comma), ';' => Ok(Token::SemiColon),
            '=' => Ok(Token::Assign),
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
    assert_eq!(gettok_str("let"), Ok(Token::Let));
}

#[test]
fn test_misc() {
    assert_eq!(gettok_str(""), Err(LexerError::EOF));
    assert_eq!(gettok_str("("), Ok(Token::LParen));
    assert_eq!(gettok_str(")"), Ok(Token::RParen));
    assert_eq!(gettok_str("{"), Ok(Token::LBrack));
    assert_eq!(gettok_str("}"), Ok(Token::RBrack));
    assert_eq!(gettok_str(","), Ok(Token::Comma));
    assert_eq!(gettok_str(";"), Ok(Token::SemiColon));
    assert_eq!(gettok_str("="), Ok(Token::Assign));
}
