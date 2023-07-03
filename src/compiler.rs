use std::collections::HashMap;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use string_builder::Builder;

use crate::{
    chunk::{Chunk, OpCode, Value},
    scanner::{Scanner, Token, TokenType},
};

struct Parser<'a> {
    current: Token,
    previous: Token,
    scanner: Scanner<'a>,
    chunk: Chunk,
}

#[derive(Debug, FromPrimitive)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

type ParseFn = dyn Fn(&Parser) -> ();

struct ParseRule {
    prefix: Option<Box<ParseFn>>,
    infix: Option<Box<ParseFn>>,
    precedence: Precedence,
}

impl<'a> Parser<'a> {
    fn new(source: &'a Vec<char>) -> Parser<'a> {
        return Parser {
            current: Token::empty(),
            previous: Token::empty(),
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
        };
    }

    fn advance(&mut self) -> Option<String> {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();

            if self.current.token_type != TokenType::Error {
                return None;
            };
            let err = self.error_at_current(&self.current.lexeme);
            return Some(err);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(&message);
    }

    fn expression(&self) {
        self.parse_precendence(Precedence::Assignment);
    }

    fn end_compiler(&self) {
        self.emit_return();
    }

    fn emit_byte(&self, byte: u8) {
        self.chunk.write(byte, self.previous.line);
    }

    fn emit_bytes(&self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn error_at_current(self, message: &str) -> String {
        return Parser::error_at(&self.current, message);
    }

    fn error(&mut self, message: &str) -> String {
        return Parser::error_at(&self.previous, message);
    }

    fn error_at(token: &Token, message: &str) -> String {
        let mut builder = Builder::default();

        builder.append(format!("[line {}] Error", token.line));

        match token.token_type {
            TokenType::EOF => builder.append(" at end"),
            TokenType::Error => {} // Nothing
            _ => builder.append(format!(" at '{}'", token.lexeme)),
        }

        builder.append(format!(": {}", message));
        return  builder.string().unwrap();
    }

    fn number(&mut self) {
        let value: Value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return;
        }

        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_owned(),
        );
    }

    fn unary(&self) {
        let operator_type = &self.previous.token_type;

        // Compile the operand.
        self.parse_precendence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => panic!("Unrecognized unary operator!"),
        }
    }

    fn binary(&self) {
        let operator_type = &self.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precendence(Precedence::from_u8((rule.precedence as u8) + 1).unwrap());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => panic!("Unrecognized binary operator!"),
        }
    }

    fn parse_precendence(&self, precedence: Precedence) {}

    fn get_rule(&self, operator_type: &TokenType) -> ParseRule {
       // return rules[operator_type];
       todo!();
    }
}

pub fn compile(source: &Vec<char>) -> Option<Chunk> {
    let mut parser = Parser::new(source);

    let result = parser.advance();
    if let Some(error) = result {
        eprintln!("{}", error);
        return None;
    }

    parser.expression();

    parser.consume(TokenType::EOF, "Expect end of expression.".to_owned());
    parser.end_compiler();

    return Some(parser.chunk);
}


// const rules: HashMap<TokenType, ParseRule> = HashMap::from([
//     (
//         TokenType::LeftParen,
//         ParseRule {
//             prefix: Some(Box::new(Parser::grouping)),
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::RightParen,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::LeftBrace,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::RightBrace,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Comma,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Dot,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Minus,
//         ParseRule {
//             prefix: Some(Box::new(Parser::unary)),
//             infix: Some(Box::new(Parser::binary)),
//             precedence: Precedence::Term,
//         },
//     ),
//     (
//         TokenType::Plus,
//         ParseRule {
//             prefix: None,
//             infix: Some(Box::new(Parser::binary)),
//             precedence: Precedence::Term,
//         },
//     ),
//     (
//         TokenType::Semicolon,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Slash,
//         ParseRule {
//             prefix: None,
//             infix: Some(Box::new(Parser::binary)),
//             precedence: Precedence::Factor,
//         },
//     ),
//     (
//         TokenType::Star,
//         ParseRule {
//             prefix: None,
//             infix: Some(Box::new(Parser::binary)),
//             precedence: Precedence::Factor,
//         },
//     ),
//     (
//         TokenType::Bang,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::BangEqual,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Equal,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::EqualEqual,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Greater,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::GreaterEqual,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Less,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::LessEqual,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Identifier,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::String,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Number,
//         ParseRule {
//             prefix: Some(Box::new(Parser::number)),
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::And,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Class,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Else,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::False,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::For,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Fun,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::If,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Nil,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Or,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Print,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Return,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Super,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::This,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::True,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Var,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::While,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::Error,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
//     (
//         TokenType::EOF,
//         ParseRule {
//             prefix: None,
//             infix: None,
//             precedence: Precedence::None,
//         },
//     ),
// ]);