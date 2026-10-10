use std::io;
use std::io::Write;

use yo::tokinizer;

const REPL: &'static str = ">";
const REPL_ERROR_PREFIX: &'static str = "yo :: [repl] :: ";

fn main() -> io::Result<()> {
    'repl: loop {
        print!("{} ", REPL);
        io::stdout()
            .flush()
            .expect(&format!("{}stdout flushing error", REPL_ERROR_PREFIX).to_string());

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}input reading error", REPL_ERROR_PREFIX).to_string());
        
        match input.as_str().trim() {
            "quit" => break 'repl,
            _ => {
                let mut lex = tokinizer::Tokinizer::new(input.clone());
                let toks = lex.tokinize();
                println!("{:?}", toks);
            }
        }
    }
    Ok(())
}