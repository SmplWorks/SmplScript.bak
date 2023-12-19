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

    ParserInvalidCallNoLParen,
    ParserInvalidCallMissingComma,

    VMCannotConvertToNumber,
    VMCannotAssignNonVariable,
    VMCannotCallNonFunction,
    VMMismatchArgumentListLength,
    VMVariableDoesntExist,
}

pub type SRes<T> = Result<T, SError>;
