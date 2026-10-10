#[derive(Debug)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INTEGER(String),

    // Basic
    ASSIGN,    // =
    PLUS,      // +
    MINUS,     // -
    EXCL,      // !
    STAR,      // *
    SLASH,     // /

    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }

    LT,       // <
    GT,       // >
    EQ,       // ==
    NEQ,      // !=

    // Keywords
    DEFINE,  // define
    YO,      // yo
    TRUE,    // true
    FALSE,   // false
    IF,      // if
    ELSE,    // else 
    RET      // ret
}