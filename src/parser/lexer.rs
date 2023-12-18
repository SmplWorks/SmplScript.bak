use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
}

pub type LexerRes<T> = Result<T, LexerError>;

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

pub fn gettok(chars : &mut Chars) -> LexerRes<Token> {
    let c = skip_whitespace(chars)?;
    if c.is_digit(10) {
        return get_number(c, chars)
    } else {
        todo!("")
    }
}

#[test]
fn test_get_number() {
    assert_eq!(gettok(&mut "0".chars()), Ok(Token::Number("0".to_string())));
    assert_eq!(gettok(&mut " 1".chars()), Ok(Token::Number("1".to_string())));
    assert_eq!(gettok(&mut "2  3".chars()), Ok(Token::Number("2".to_string())));
}
