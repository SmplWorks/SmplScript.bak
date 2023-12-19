mod tokenizer;
mod token;
mod error;
pub use tokenizer::*;
pub use token::*;
pub use error::*;

#[test]
fn test_full_function() {
    use tokenizer::tokenize;
    use token::Token;

    let code = "\
fn main(x, y) {
    return 1
}";

    assert_eq!(tokenize(code.chars()).collect::<Vec<_>>(), vec![
        Token::Function, Token::Identifier("main".to_string()), Token::LParen,
            Token::Identifier("x".to_string()), Token::Comma, Token::Identifier("y".to_string()),
        Token::RParen, Token::LBrack,
            Token::Return, Token::Number("1".to_string()),
        Token::RBrack,
    ]);
}
