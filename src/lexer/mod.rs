pub mod tokenizer;
pub mod token;
pub mod error;

#[test]
fn test_full() {
    use tokenizer::tokenize;
    use token::Token;

    let code = "\
fn main() {
    return 1
}";

    assert_eq!(tokenize(&mut code.chars()).collect::<Vec<_>>(), vec![
        Token::Function, Token::Identifier("main".to_string()), Token::LParen, Token::RParen, Token::LBrack,
            Token::Return, Token::Number("1".to_string()),
        Token::RBrack,
    ]);
}
