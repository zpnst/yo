use std::fs;
use std::io;

use yo::lexer;

fn main() {
    let input = read_file("expls/Debug.yo");
    let mut lex = lexer::Lexer::new(input.clone());
    let toks = lex.tokinize();
    println!("{:?}", toks);
}

fn read_file(path: &str) -> String {
    let r_input: io::Result<String> = fs::read_to_string(path);
    match r_input {
        Ok(i) => i,
        Err(err) => panic!("{:?}", err)
    }
}