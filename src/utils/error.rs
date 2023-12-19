#[derive(Debug, PartialEq)]
pub enum SError {
    LexerEOF,
    LexerUnknownToken,

}

pub type SRes<T> = Result<T, SError>;
