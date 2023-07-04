use num_derive::FromPrimitive;

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

impl Token {
    pub fn empty() -> Token {
        return Token {
            token_type: TokenType::Error,
            lexeme: "Empty token.".to_owned(),
            line: -1,
        };
    }

    pub fn clone(&self) -> Self {
        Token {
            token_type: self.token_type.clone(),
            lexeme: self.lexeme.clone(),
            line: self.line,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, FromPrimitive, Clone)]
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

        return match c {
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
            }
            '"' => self.make_string(),
            a if self.is_alpha(a) => self.make_identifier(),
            d if self.is_digit(d) => self.make_number(),
            _ => self.error_token(&format!("Unexpected character: {}.", c)),
        };
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() - 1;
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        return Token {
            token_type: token_type,
            lexeme: self.source[self.start..self.current].iter().collect(),
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
        while !self.is_at_end() {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
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

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn make_identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.source[self.current];

        return match c {
            'a' => self.check_keyword("and", TokenType::And),
            'c' => self.check_keyword("class", TokenType::Class),
            'e' => self.check_keyword("else", TokenType::Else),
            'f' if self.current - self.start > 1 => match self.source[self.start + 1] {
                'a' => self.check_keyword("false", TokenType::False),
                'o' => self.check_keyword("for", TokenType::For),
                'u' => self.check_keyword("fun", TokenType::Fun),
                _ => TokenType::Identifier,
            },
            'i' => self.check_keyword("if", TokenType::If),
            'n' => self.check_keyword("nil", TokenType::Nil),
            'o' => self.check_keyword("or", TokenType::Or),
            'p' => self.check_keyword("print", TokenType::Print),
            'r' => self.check_keyword("return", TokenType::Return),
            's' => self.check_keyword("super", TokenType::Super),
            't' if self.current - self.start > 1 => match self.source[self.start + 1] {
                'h' => self.check_keyword("this", TokenType::This),
                'r' => self.check_keyword("true", TokenType::True),
                _ => TokenType::Identifier,
            },
            'v' => self.check_keyword("var", TokenType::Var),
            'w' => self.check_keyword("while", TokenType::While),
            _ => TokenType::Identifier,
        };
    }

    fn check_keyword(&self, expected: &str, token_type: TokenType) -> TokenType {
        let actual: String = self.source[self.start..=self.current].iter().collect();
        if actual == expected {
            return token_type;
        }

        return TokenType::Identifier;
    }
}
