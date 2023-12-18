use std::str::Chars;
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

fn skip_whitespace(chars : &mut Chars) -> LexerRes<char> {
    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            return Ok(c)
        }
    }
    Err(LexerError::EOF)
}

fn get_number(c : char, chars : &mut Chars) -> LexerRes<Token> {
    // TODO: Different bases and '_'
    Ok(Token::Number(c.to_string() + &chars.take_while(|c| c.is_digit(10)).collect::<String>()))
}

fn get_ident(c : char, chars : &mut Chars) -> LexerRes<Token> {
    let ident = c.to_string() + &chars.take_while(|c| c.is_alphanumeric() || c == &'_').collect::<String>();
    Ok(match &*ident {
        "fn" => Token::Function,
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    })
}

pub fn gettok(chars : &mut Chars) -> LexerRes<Token> {
    let c = skip_whitespace(chars)?;
    if c.is_digit(10) {
        return get_number(c, chars)
    } else if c.is_alphabetic() {
        return get_ident(c, chars)
    } else {
        match c {
            '(' => Ok(Token::LParen), ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrack), '}' => Ok(Token::RBrack),
            _ => todo!(),
        }
    }
}

#[test]
fn test_get_number() {
    assert_eq!(gettok(&mut "0".chars()), Ok(Token::Number("0".to_string())));
    assert_eq!(gettok(&mut " 1".chars()), Ok(Token::Number("1".to_string())));
    assert_eq!(gettok(&mut "2  3".chars()), Ok(Token::Number("2".to_string())));
}

#[test]
fn test_get_identifier() {
    assert_eq!(gettok(&mut "g".chars()), Ok(Token::Identifier("g".to_string())));
    assert_eq!(gettok(&mut "galileo".chars()), Ok(Token::Identifier("galileo".to_string())));
    assert_eq!(gettok(&mut "gali leo".chars()), Ok(Token::Identifier("gali".to_string())));
    assert_eq!(gettok(&mut "g4l1l30".chars()), Ok(Token::Identifier("g4l1l30".to_string())));
    assert_eq!(gettok(&mut "gali_leo".chars()), Ok(Token::Identifier("gali_leo".to_string())));

    assert_eq!(gettok(&mut "fn".chars()), Ok(Token::Function));
    assert_eq!(gettok(&mut "return".chars()), Ok(Token::Return));
}
