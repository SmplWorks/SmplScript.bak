#[derive(Debug, PartialEq)]
pub enum LexerError {
    EOF,
    UnknownToken,
}

pub type LexerRes<T> = Result<T, LexerError>;
