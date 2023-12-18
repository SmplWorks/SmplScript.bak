#[derive(Debug, PartialEq)]
pub enum LexerError {
    EOF,
}

pub type LexerRes<T> = Result<T, LexerError>;
