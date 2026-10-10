pub mod keywords;

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    plain_input: String,
    curr_position: usize,
    curr_symbol: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
       Self { 
            plain_input: input.clone(), 
            curr_position: usize::default(), 
            curr_symbol: input.as_bytes()[0]
        }
    }

    pub fn tokinize(&mut self) -> Vec<Token> {
        if self.plain_at(self.plain_input.len()-1) != b'\n' {
            self.plain_input.push('\n');
        }
        let mut res = Vec::new();
        while self.curr_symbol != 0 {
            res.push(self.next_token());
        }
        res
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespaces_and_control_symbols();

        let res: Token = match self.curr_symbol {
            0    => Token::EOF,
            b'+' => Token::PLUS,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'-' => Token::MINUS,
            b'*' => Token::STAR,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'!' => {
                if self.next_symbol() == b'=' {
                    self.read_symbol();
                    Token::NEQ 
                } else {
                    Token::EXCL
                }
            },
            b'=' => {
                if self.next_symbol() == b'=' {
                    self.read_symbol();
                    Token::EQ 
                } else {
                    Token::ASSIGN
                }
            },
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
        self.read_symbol();
        res
    }

    fn skip_whitespaces_and_control_symbols(&mut self) {
        while
            self.curr_symbol == b' '  || self.curr_symbol == b'\n'  || 
            self.curr_symbol == b'\t'  || self.curr_symbol == b'\r'
        {
            self.read_symbol();
        }
    }

    fn read_symbol(&mut self) {
        if self.curr_position+1 >= self.plain_input.len() {
            self.curr_symbol = 0;
        } else {
            self.curr_symbol = self.plain_at(self.curr_position+1);
        }
        self.curr_position += 1;
    }

    fn next_symbol(&mut self) -> u8 {
        if self.curr_position+1 >= self.plain_input.len() {
            return 0;
        } else {
            return self.plain_at(self.curr_position+1)
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_curr_position = self.curr_position;
        while is_letter(self.curr_symbol) {
            self.read_symbol();
        }
        keywords::filter(self.plain_input[start_curr_position..self.curr_position].to_string())
    }

    fn read_integer(&mut self) -> Token {
        let start_curr_position = self.curr_position;
        while is_integer(self.curr_symbol) {
            self.read_symbol();
        }
        Token::INTEGER(
            self.plain_input[start_curr_position..self.curr_position].to_string()
        )
    }

    fn plain_at(&self, at: usize) -> u8 {
        self.plain_input.as_bytes()[at]
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
