#[derive(Debug, PartialEq)]
pub enum ParserError {
    EOF,

    InvalidNumber,

    InvalidFunctionNoName,
    InvalidFunctionNoLParen,
    InvalidFunctionMissingComma,
    InvalidFunctionExtraComma,
    InvalidFunctionExpectedParam,
    InvalidFunctionInvalidToken,
}

pub type ParserRes<T> = Result<T, ParserError>;
