use std::vec;

use yo::tokinizer;

use yo::token::Token;

const BASR_DIR: &'static str = "tests/cases";
const TESTS_ERROR_PREFIX: &'static str = "yo :: [tests] :: ";

#[test]
fn test_basic() {
    let input: String = read_file("Basic")
        .expect(&format!("{}read_file error", TESTS_ERROR_PREFIX).to_string());

    let token_vec: Vec<Token> = vec![
        Token::YO,
        Token::IDENT("x".to_string()), 
        Token::ASSIGN, 
        Token::INTEGER("1".to_string()), 
        Token::SEMICOLON, 
        Token::YO, 
        Token::IDENT("y".to_string()), 
        Token::ASSIGN, 
        Token::INTEGER("41".to_string()), 
        Token::SEMICOLON, 
        Token::YO, 
        Token::IDENT("add".to_string()), 
        Token::ASSIGN, 
        Token::DEFINE, 
        Token::LPAREN, 
        Token::IDENT("x".to_string()), 
        Token::COMMA, 
        Token::IDENT("y".to_string()), 
        Token::RPAREN, 
        Token::LBRACE, 
        Token::IDENT("x".to_string()), 
        Token::PLUS, 
        Token::IDENT("y".to_string()), 
        Token::SEMICOLON, 
        Token::RBRACE, 
        Token::SEMICOLON, 
        Token::YO, 
        Token::IDENT("res".to_string()), 
        Token::ASSIGN, 
        Token::IDENT("add".to_string()), 
        Token::LPAREN, 
        Token::IDENT("x".to_string()), 
        Token::COMMA, 
        Token::IDENT("y".to_string()), 
        Token::RPAREN,
        Token::SEMICOLON, 
        Token::EOF
    ];

    let mut lex = tokinizer::Tokinizer::new(input.clone());
    let toks = lex.tokinize();

    assert_eq!(toks, token_vec);
}

#[test]
fn test_2token() {
    let input: String = read_file("2Token")
        .expect(&format!("{}read_file error", TESTS_ERROR_PREFIX).to_string());

    let token_vec: Vec<Token> = vec![
        Token::IF, 
        Token::IDENT("x".to_string()), 
        Token::EQ, 
        Token::IDENT("y".to_string()), 
        Token::LBRACE, 
        Token::YO, 
        Token::IDENT("pred".to_string()), 
        Token::ASSIGN, 
        Token::IDENT("x".to_string()), 
        Token::NEQ, 
        Token::INTEGER("42".to_string()), 
        Token::SEMICOLON, 
        Token::RBRACE, 
        Token::SEMICOLON, 
        Token::EOF
    ];

    let mut lex = tokinizer::Tokinizer::new(input.clone());
    let toks = lex.tokinize();

    assert_eq!(toks, token_vec);
}

fn read_file(f: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(format!("{}/{}.yo", BASR_DIR, f))
}