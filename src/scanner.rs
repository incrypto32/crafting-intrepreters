use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    String(String),
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
                '.' => self.add_token(Token::Dot),
                '-' => self.add_token(Token::Minus),
                '+' => self.add_token(Token::Plus),
                ';' => self.add_token(Token::SemiColon),
                '*' => self.add_token(Token::Star),

                // Scanning operators these are multi characters
                // So we need to handle it appropriately
                // Eat the characters until we find a valid lexeme
                // We dont advance current if the next char does not match
                // This is because we need to process the char again in the next natural advance
                '!' => {
                    if self.match_char('=') {
                        self.add_token(Token::BangEqual)
                    } else {
                        self.add_token(Token::Bang)
                    }
                }

                '=' => {
                    if self.match_char('=') {
                        self.add_token(Token::EqualEqual)
                    } else {
                        self.add_token(Token::Equal)
                    }
                }

                '<' => {
                    if self.match_char('=') {
                        self.add_token(Token::LessEqual)
                    } else {
                        self.add_token(Token::Less)
                    }
                }

                '>' => {
                    if self.match_char('=') {
                        self.add_token(Token::GreaterEqual)
                    } else {
                        self.add_token(Token::Greater)
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
                        self.add_token(Token::Slash);
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
                    } else if self.is_alpha_numeric(Some(c)) {
                        // Pass Some(c) here
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
        while self.peek().is_some() && self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        // Consume the closing quote
        self.advance();

        // Extract the string value, excluding the quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(Token::String(value));
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn error(&mut self, line: usize, message: &str) {
        self.has_error = true;
        eprintln!("[line {}] Error: {}", line, message); // Changed to eprintln!
    }

    // Public getter for has_error
    pub fn has_error(&self) -> bool {
        self.has_error
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::SemiColon => write!(f, ";"),
            Token::Star => write!(f, "*"),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Slash => write!(f, "/"),
            Token::String(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier => write!(f, "identifier"),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn testit() {
        let code = "{} () 23454 + 23.4 - 23.4 + // Hi\n +";
        let mut scanner = super::Scanner::new(code.to_string());

        let tokens = scanner.scan_tokens();
        let expected = vec![
            super::Token::LeftBrace,
            super::Token::RightBrace,
            super::Token::LeftParen,
            super::Token::RightParen,
            super::Token::Number(23454.0),
            super::Token::Plus,
            super::Token::Number(23.4),
            super::Token::Minus,
            super::Token::Number(23.4),
            super::Token::Plus,
            super::Token::Plus,
            super::Token::Eof,
        ];
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens, expected);
    }
}
