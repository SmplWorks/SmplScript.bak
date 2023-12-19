#[derive(Debug, PartialEq)]
pub enum ParserError {
    EOF,
    InvalidNumber,
    InvalidFunction(&'static str),
}

pub type ParserRes<T> = Result<T, ParserError>;
