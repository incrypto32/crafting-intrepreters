use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    SemiColon,
    Star,
    Slash,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    True,
    False,
    Nil,

    // End of file.
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl LiteralValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::Boolean(b) => *b,
            LiteralValue::Nil => false,
            _ => true,
        }
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
}

impl Token {
    /// Convenience constructor for simple, punctuation-style tokens with no literal.
    pub fn simple(typ: TokenType, lexeme: &str, line: usize) -> Self {
        Token {
            typ,
            lexeme: lexeme.to_string(),
            literal: None,
            line,
        }
    }

    /// Convenience constructor for tokens that carry a literal value.
    pub fn with_literal(typ: TokenType, lexeme: &str, literal: LiteralValue, line: usize) -> Self {
        Token {
            typ,
            lexeme: lexeme.to_string(),
            literal: Some(literal),
            line,
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let s = match self {
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            SemiColon => ";",
            Star => "*",
            Slash => "/",
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Less => "<",
            LessEqual => "<=",
            Greater => ">",
            GreaterEqual => ">=",
            Identifier => "identifier",
            String => "string",
            Number => "number",
            True => "true",
            False => "false",
            Nil => "nil",
            Eof => "EOF",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.lexeme.is_empty() {
            write!(f, "{}", self.lexeme)
        } else {
            // Fallback to printing token type when lexeme is not available.
            write!(f, "{}", self.typ)
        }
    }
}
