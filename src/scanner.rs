#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    EOF,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    SLASH,
    STRING(String),
    NUMBER(f64),
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            source,
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::EOF);
        self.tokens.clone()
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;
        result
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    pub fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(Token::LEFT_PAREN),
                ')' => self.add_token(Token::RIGHT_PAREN),
                '{' => self.add_token(Token::LEFT_BRACE),
                '}' => self.add_token(Token::RIGHT_BRACE),
                ',' => self.add_token(Token::COMMA),
                '.' => self.add_token(Token::DOT),
                '-' => self.add_token(Token::MINUS),
                '+' => self.add_token(Token::PLUS),
                ';' => self.add_token(Token::SEMICOLON),
                '*' => self.add_token(Token::STAR),

                // Scanning operators these are multi characters
                // So we need to handle it appropriately
                // Eat the characters until we find a valid lexeme
                // We dont advance current if the next char does not match
                // This is because we need to process the char again in the next natural advance
                '!' => {
                    if self.match_char('=') {
                        self.add_token(Token::BANG_EQUAL)
                    } else {
                        self.add_token(Token::BANG)
                    }
                }

                '=' => {
                    if self.match_char('=') {
                        self.add_token(Token::EQUAL_EQUAL)
                    } else {
                        self.add_token(Token::EQUAL)
                    }
                }

                '<' => {
                    if self.match_char('=') {
                        self.add_token(Token::LESS_EQUAL)
                    } else {
                        self.add_token(Token::LESS)
                    }
                }

                '>' => {
                    if self.match_char('=') {
                        self.add_token(Token::GREATER_EQUAL)
                    } else {
                        self.add_token(Token::GREATER)
                    }
                }

                '/' => {
                    if self.match_char('/') {
                        // Ignore comments
                        // We consume characters until we find a newline
                        while matches!(self.peek(), Some(c) if c != '\n') {
                            self.advance();
                        }
                    } else {
                        self.add_token(Token::SLASH);
                    }
                }

                // Ignore whitespace
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                // String literals
                '"' => self.string(),

                // Longer Lexemes
                _ => {
                    if c.is_ascii_digit() {
                        self.scan_number();
                    } else {
                        self.error(self.line, "Unexpected character.");
                    }
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }

    fn scan_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token(Token::NUMBER(value.parse().unwrap()));
    }

    fn string(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => {
                    if self.is_at_end() {
                        self.error(self.line, "Unterminated string.");
                        return;
                    }
                    self.advance();
                }
            }
        }

        // Consume the closing quote
        self.advance();

        // Extract the string value, excluding the quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(Token::STRING(value));
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn error(&mut self, line: usize, message: &str) {
        self.has_error = true;
        println!("Error: {} at line {}", message, line);
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn testit() {
        let code = "{} () 23454 + 23.4 - 23.4";
        let mut scanner = super::Scanner::new(code.to_string());

        let tokens = scanner.scan_tokens();
        let expected = vec![
            super::Token::LEFT_BRACE,
            super::Token::RIGHT_BRACE,
            super::Token::LEFT_PAREN,
            super::Token::RIGHT_PAREN,
            super::Token::NUMBER(23454.0),
            super::Token::PLUS,
            super::Token::NUMBER(23.4),
            super::Token::MINUS,
            super::Token::NUMBER(23.4),
            super::Token::EOF,
        ];
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens, expected);
    }
}
