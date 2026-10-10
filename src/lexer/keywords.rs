use crate::token::Token;

pub fn filter(ik: String) -> Token {
    match ik.as_str() {
        "define" => Token::DEFINE,
        "yo"     => Token::YO,
        "true"   => Token::TRUE,
        "false"  => Token::FALSE,
        "if"     => Token::IF,
        "else"   => Token::ELSE,
        "ret"    => Token::RET,
        _        => Token::IDENT(ik)
    }
}