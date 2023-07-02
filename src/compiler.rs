use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &Vec<char>) {
    let mut scanner = Scanner::new(source);

    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:?} '{}'", token.token_type, token.lexeme);

        if token.token_type == TokenType::EOF {
            break;
        } 
    }
}