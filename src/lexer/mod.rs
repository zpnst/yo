use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    plain_input: String,
    current_position: usize,
    current_character: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
       Self { 
            plain_input: input.clone(), 
            current_position: usize::default(), 
            current_character: input.as_bytes()[0]
        }
    }

    pub fn tokinize(&mut self) -> Vec<Token> {
        if self.plain_input.as_bytes()[self.plain_input.len()-1] != b'\n' {
            self.plain_input.push('\n');
        }
        let mut res = Vec::new();
        while self.current_character != 0 {
            res.push(self.next_token());
        }
        res
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespaces_and_control_characters();

        let res: Token = match self.current_character {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0    => Token::EOF,
            some => {
                if is_letter(some) {
                    return self.read_identifier();
                } else if is_integer(some) {
                    return self.read_integer();
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_character();
        res
    }

    fn skip_whitespaces_and_control_characters(&mut self) {
        while
            self.current_character == b' '  || self.current_character == b'\n'  || 
            self.current_character == b'\t'  || self.current_character == b'\r'
        {
            self.read_character();
        }
    }

    fn read_character(&mut self) {
        if self.current_position+1 >= self.plain_input.len() {
            self.current_character = 0;
        } else {
            self.current_character = self.plain_input.as_bytes()[self.current_position+1];
        }
        self.current_position += 1;
    }

    fn read_identifier(&mut self) -> Token {
        let start_current_position = self.current_position;
        while is_letter(self.current_character) {
            self.read_character();
        }
        keyword_filter(self.plain_input[start_current_position..self.current_position].to_string())
    }

    fn read_integer(&mut self) -> Token {
        let start_current_position = self.current_position;
        while is_integer(self.current_character) {
            self.read_character();
        }
        Token::INTEGER(
            self.plain_input[start_current_position..self.current_position].to_string()
        )
    }

}

fn keyword_filter(ik: String) -> Token {
    match ik.as_str() {
        "define" => Token::DEFINE,
        "yo"     => Token::YO,
        _        => Token::IDENT(ik)
    }
}

fn is_letter(il: u8) -> bool {
    il >= b'a' && il <= b'z' || 
    il >= b'A' && il <= b'Z' || 
   il == b'_'
}

fn is_integer(ii: u8) -> bool {
    ii >= b'0' && ii <= b'9'
}
