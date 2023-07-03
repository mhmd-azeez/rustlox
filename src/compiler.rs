use string_builder::Builder;

use crate::{scanner::{Scanner, TokenType, Token}, chunk::{Chunk, OpCode}};

struct Parser<'a> {
    current: Token,
    previous: Token,
    scanner: Scanner<'a>,
    chunk: Chunk
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Parser<'a> {
        return Parser{
            current: Token::empty(),
            previous: Token::empty(),
            scanner,
            chunk: Chunk::new(),
        };
    }

    fn advance(&self) -> Option<String> {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            if self.current.token_type != TokenType::Error {
                return None;
            };
            
            return Some(self.error_at_current(self.current.lexeme));
        }
    }

    fn consume(&self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emitByte(&self, byte: u8) {
        self.current_chunk().write(byte, self.previous.line);
    }

    fn emitBytes(&self, byte1: u8, byte2: u8) {
        self.emitByte(byte1);
        self.emitByte(byte2);
    }

    fn emit_return(&self) {
        self.emitByte(OpCode::Return as u8);
    }

    fn current_chunk(&self) -> Chunk {
        return self.chunk;
    }

    fn error_at_current(self, message: String) -> String {
        return Parser::error_at(self.current, message);
    }

    fn error(self, message: String) -> String {
        return Parser::error_at(self.previous, message);
    }

    fn error_at(token: Token, message: String) -> String {
        let mut builder = Builder::default();

        builder.append(format!("[line {}] Error", token.line));

        match token.token_type {
            TokenType::EOF => builder.append(" at end"),
            TokenType::Error => {}, // Nothing
            _ => builder.append(format!(" at '{}'", token.lexeme)),
        }

        builder.append(format!(": {}", message))
    }

    fn end_compiler(&self) {
        self.emit_return();
    }
}

pub fn compile(source: &Vec<char>) -> Option<Chunk> {
    let mut scanner = Scanner::new(source);
    let parser = Parser::new(scanner);

    let result = parser.advance();
    if let Some(error) = result {
        eprintln!("{}", error);
        return None;
    }

    expression();

    parser.consume(TokenType::EOF, "Expect end of expression.".to_owned());
    parser.end_compiler();

    return Some(parser.chunk);
}