use std::{str::Chars, iter::Peekable};
use super::{Token, gettok};

pub struct Tokens<'a> {
    chars : Peekable<Chars<'a>>,
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        gettok(&mut self.chars).ok()
    }
}

pub fn tokenize(chars : Chars) -> Tokens {
    Tokens{ chars: chars.peekable() }
}
