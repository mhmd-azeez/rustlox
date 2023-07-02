pub struct Scanner<'a> {
    source: &'a Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    EOF,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &Vec<char>) -> Scanner {
        return Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

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
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            },
            '"' => self.make_string(),
            d if self.is_digit(c) => self.make_number(),
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
            lexeme: self.source[self.start..=self.current].iter().collect(),
            line: self.line,
        };
    }

    fn error_token(&self, message: &str) -> Token {
        return Token {
            token_type: TokenType::Error,
            lexeme: message.to_string(),
            line: self.line,
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source[self.current - 1];
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                             self.advance();
                        }
                    }
                }
                _ => return
            }
        }
    }

    fn peek(&self) -> char {
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source[self.current + 1];
      }

    fn make_string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        return self.make_token(TokenType::String);
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn make_number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the '.'
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }
}
