// use std::fmt; // REMOVE
use crate::token::{LiteralValue, Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        // Final EOF token.
        self.tokens.push(Token::simple(TokenType::Eof, "", self.line));
        self.tokens.clone()
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(ch) => ch,
            None => return,
        };

        match c {
            '(' => self.add_simple(TokenType::LeftParen),
            ')' => self.add_simple(TokenType::RightParen),
            '{' => self.add_simple(TokenType::LeftBrace),
            '}' => self.add_simple(TokenType::RightBrace),
            ',' => self.add_simple(TokenType::Comma),
            '.' => self.add_simple(TokenType::Dot),
            '-' => self.add_simple(TokenType::Minus),
            '+' => self.add_simple(TokenType::Plus),
            ';' => self.add_simple(TokenType::SemiColon),
            '*' => self.add_simple(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_simple(TokenType::BangEqual);
                } else {
                    self.add_simple(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_simple(TokenType::EqualEqual);
                } else {
                    self.add_simple(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_simple(TokenType::LessEqual);
                } else {
                    self.add_simple(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_simple(TokenType::GreaterEqual);
                } else {
                    self.add_simple(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // comment till end of line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_simple(TokenType::Slash);
                }
            }
            // whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            // string literal
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    self.error("Unexpected character.");
                }
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.source.chars().nth(self.current);
        self.current += 1;
        ch
    }

    fn add_simple(&mut self, typ: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::simple(typ, &lexeme, self.line));
    }

    fn add_literal(&mut self, typ: TokenType, literal: LiteralValue) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::with_literal(typ, &lexeme, literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn string(&mut self) {
        while self.peek().is_some() && self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
            return;
        }

        // closing quote
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_literal(TokenType::String, LiteralValue::String(value));
    }

    fn number(&mut self) {
        while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }

        // fractional part
        if self.peek() == Some('.') && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        {
            self.advance(); // consume '.'
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
        }

        let value_str = &self.source[self.start..self.current];
        let number: f64 = value_str.parse().unwrap_or(0.0);
        self.add_literal(TokenType::Number, LiteralValue::Number(number));
    }

    fn identifier(&mut self) {
        while self.peek().map(Self::is_alphanumeric).unwrap_or(false) {
            self.advance();
        }

        // Determine if identifier is a reserved keyword.
        let text = &self.source[self.start..self.current];
        let typ = match text {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            _ => TokenType::Identifier,
        };

        self.tokens.push(Token::simple(typ, text, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn error(&mut self, message: &str) {
        self.has_error = true;
        eprintln!("[line {}] Error: {}", self.line, message);
    }

    // Helper character classification functions.
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
