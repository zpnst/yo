#[derive(Debug)]
pub(crate) enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INTEGER(String),

    // Basic
    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    DEFINE,
    YO
}