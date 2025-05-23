#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    LESS,
    LessEqual,
    GREATER,
    GreaterEqual,
    SLASH,
    STRING(String),
    Number(f64),
    Identifier,
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

        self.tokens.push(Token::Eof);
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
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                '{' => self.add_token(Token::LeftBrace),
                '}' => self.add_token(Token::RightBrace),
                ',' => self.add_token(Token::Comma),
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
                        self.add_token(Token::BangEqual)
                    } else {
                        self.add_token(Token::BANG)
                    }
                }

                '=' => {
                    if self.match_char('=') {
                        self.add_token(Token::EqualEqual)
                    } else {
                        self.add_token(Token::EQUAL)
                    }
                }

                '<' => {
                    if self.match_char('=') {
                        self.add_token(Token::LessEqual)
                    } else {
                        self.add_token(Token::LESS)
                    }
                }

                '>' => {
                    if self.match_char('=') {
                        self.add_token(Token::GreaterEqual)
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
                    } else if self.is_alpha_numeric(c) {
                        self.identifier();
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

    fn is_alpha(&self, c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_ascii_alphabetic() || c == '_',
            None => false,
        }
    }

    fn is_alpha_numeric(&self, c: Option<char>) -> bool {
        self.is_alpha(c) || self.is_digit(c)
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
        self.add_token(Token::Number(value.parse().unwrap()));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(Token::Identifier);
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
            super::Token::LeftBrace,
            super::Token::RightBrace,
            super::Token::LeftParen,
            super::Token::RightParen,
            super::Token::Number(23454.0),
            super::Token::PLUS,
            super::Token::Number(23.4),
            super::Token::MINUS,
            super::Token::Number(23.4),
            super::Token::Eof,
        ];
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens, expected);
    }
}
