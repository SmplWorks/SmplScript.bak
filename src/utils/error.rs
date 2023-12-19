#[derive(Debug, PartialEq)]
pub enum SError {
    LexerEOF,
    LexerUnknownToken,

    ParserExpectedClosingParen,

    ParserInvalidNumber,

    ParserInvalidFunctionNoName,
    ParserInvalidFunctionNoLParen,
    ParserInvalidFunctionMissingComma,
    ParserInvalidFunctionExtraComma,
    ParserInvalidFunctionExpectedParam,
    ParserInvalidFunctionInvalidToken,

    VMCannotAssignNonVariable,
}

pub type SRes<T> = Result<T, SError>;
