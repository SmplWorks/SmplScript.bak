#[derive(Debug, PartialEq)]
pub enum ParserError {
    EOF,
    InvalidNumber,
}

pub type ParserRes<T> = Result<T, ParserError>;
