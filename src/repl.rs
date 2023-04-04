use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::token::Token;

const PROMPT: &str = ">> ";

pub fn start() {
    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}
