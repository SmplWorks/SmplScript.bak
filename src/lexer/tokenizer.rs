use std::str::Chars;
use super::token::*;

pub struct Tokens<'a> {
    chars : &'a mut Chars<'a>,
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        gettok(self.chars).ok()
    }
}

pub fn tokenize<'a>(chars : &'a mut Chars<'a>) -> Tokens<'a> {
    Tokens{ chars }
}
