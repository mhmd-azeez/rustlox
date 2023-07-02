pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    Error, EOF,
}


impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        return Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();

        let token = match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            _ => panic!("Invalid character"),
        };

        return self.error_token("Unexpected character.");
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        return Token {
            token_type: token_type,
            lexeme: self.source.get(self.start..self.current).unwrap().to_string(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        return Token {
            token_type: TokenType::Error,
            lexeme: message.to_string(),
            line: self.line,
        }
    }

    fn advance(&self) -> char {
        self.current += 1;
        return self.source.get(self.current - 1).unwrap();
    }
}