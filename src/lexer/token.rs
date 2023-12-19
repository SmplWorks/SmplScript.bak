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
    LAnd, LOr, LNot, // and or not

    LParen, RParen, // ( )
    LBrack, RBrack, // { }
    Comma, SemiColon, // , ;
    Assign, // =
    Not, // !
    Add, Sub, // + -
    Mul, Div, // * /
    Equals, Nequals, // == !=
    LeThan, LeqThan, // < <=
    GeThan, GeqThan, // > >=
}

impl Token {
    pub fn is_binary_operator(&self) -> bool {
        match self {
            Self::Assign | Self::LAnd | Self::LOr | Self::LNot | Self::Add |
            Self::Sub | Self::Mul | Self::Div | Self::Equals | Self::Nequals | Self::LeThan | Self::LeqThan |
            Self::GeThan | Self::GeqThan => true,
            _ => false,
        }
    }

    pub fn get_precedence(&self) -> i32 {
        match self {
            Self::Assign => 2,
            Self::LAnd | Self::LOr => 9,
            Self::Equals | Self::Nequals | Self::LeThan | Self::LeqThan | Self::GeThan | Self::GeqThan => 10,
            Self::Add | Self::Sub => 20,
            Self::Mul | Self::Div => 40,
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
            Self::LAnd => write!(f, "and"),
            Self::LOr => write!(f, "or"),
            Self::LNot => write!(f, "not"),
            Self::LParen => write!(f, "("), Token::RParen => write!(f, ")"),
            Self::LBrack => write!(f, "{}", '{'), Token::RBrack => write!(f, "{}", '}'),
            Self::Comma => write!(f, ","), Token::SemiColon => write!(f, ";"),
            Self::Assign => write!(f, "="),
            Self::Not => write!(f, "!"),
            Self::Add => write!(f, "+"), Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"), Self::Div => write!(f, "/"),
            Self::Equals => write!(f, "=="), Self::Nequals => write!(f, "!="),
            Self::LeThan => write!(f, "<"), Self::LeqThan => write!(f, "<="),
            Self::GeThan => write!(f, ">"), Self::GeqThan => write!(f, ">="),
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
        "and" => Token::LAnd,
        "or" => Token::LOr,
        "not" => Token::LNot,
        _ => Token::Identifier(ident),
    })
}

fn foo(next : char, option_a : Token, option_b : Token, chars : &mut Peekable<Chars>) -> LexerRes<Token> {
    if let Some(c) = chars.peek() {
        if *c == next {
            chars.next();
            return Ok(option_a)
        }
    }
    return Ok(option_b)
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
            '=' => foo('=', Token::Equals, Token::Assign, chars),
            '+' => Ok(Token::Add), '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul), '/' => Ok(Token::Div),
            '!' => foo('=', Token::Nequals, Token::Not, chars),
            '<' => foo('=', Token::LeqThan, Token::LeThan, chars),
            '>' => foo('=', Token::GeqThan, Token::GeThan, chars),
            _ => Err(LexerError::UnknownToken),
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
    assert_eq!(gettok_str("and"), Ok(Token::LAnd));
    assert_eq!(gettok_str("or"), Ok(Token::LOr));
    assert_eq!(gettok_str("not"), Ok(Token::LNot));
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
    assert_eq!(gettok_str("+"), Ok(Token::Add));
    assert_eq!(gettok_str("-"), Ok(Token::Sub));
    assert_eq!(gettok_str("*"), Ok(Token::Mul));
    assert_eq!(gettok_str("/"), Ok(Token::Div));
    assert_eq!(gettok_str("=="), Ok(Token::Equals));
    assert_eq!(gettok_str("= ="), Ok(Token::Assign));
    assert_eq!(gettok_str("!="), Ok(Token::Nequals));
    assert_eq!(gettok_str("<"), Ok(Token::LeThan));
    assert_eq!(gettok_str("<="), Ok(Token::LeqThan));
    assert_eq!(gettok_str(">"), Ok(Token::GeThan));
    assert_eq!(gettok_str(">="), Ok(Token::GeqThan));
}
