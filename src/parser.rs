use std::fmt::Display;

use crate::token::{LiteralValue, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}", self.message, self.token.line)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().typ == *token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().typ, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator: operator.clone(),
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Boolean(false),
            }));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Boolean(true),
            }));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Nil,
            }));
        }

        if self.match_token(&[TokenType::String]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal.clone().unwrap(),
            }));
        }

        if self.match_token(&[TokenType::Number]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal.clone().unwrap(),
            }));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping {
                expr: Box::new(expr),
            }));
        }

        let token = self.peek().clone();
        Err(ParseError {
            token: token.clone(),
            message: format!("Expected expression, got '{}'", token.lexeme),
        })
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        message: &'static str,
    ) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let token = self.peek().clone();
            Err(ParseError {
                token,
                message: message.to_string(),
            })
        }
    }
}

// Add unit tests for the parser.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_printer::AstPrinter;
    use crate::scanner::Scanner;

    /// Helper that takes source code, scans it into tokens, parses into an AST and
    /// pretty-prints it using `AstPrinter` so that we can compare with a simple string.
    fn parse_and_print(source: &str) -> String {
        // Scan the source into tokens.
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        assert!(
            !scanner.has_error(),
            "Scanner reported an error while processing '{}'.",
            source
        );

        // Parse the tokens into an expression.
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().expect("Parser returned an error");

        // Print the AST back to a string.
        let mut printer = AstPrinter::new();
        expr.accept(&mut printer)
    }

    #[test]
    fn parses_single_number_literal() {
        assert_eq!(parse_and_print("123"), "123");
    }

    #[test]
    fn parses_unary_minus() {
        assert_eq!(parse_and_print("-123"), "(- 123)");
    }

    #[test]
    fn parses_simple_binary_expression() {
        assert_eq!(parse_and_print("1 + 2"), "(+ 1 2)");
    }

    #[test]
    fn parses_operator_precedence() {
        // '*' has higher precedence than '+', so the expression should parse as `1 + (2 * 3)`.
        assert_eq!(parse_and_print("1 + 2 * 3"), "(+ 1 (* 2 3))");
    }

    #[test]
    fn parses_grouping_expression() {
        assert_eq!(parse_and_print("(1 + 2) * 3"), "(* (group (+ 1 2)) 3)");
    }

    #[test]
    fn parses_comparison_expression() {
        assert_eq!(parse_and_print("1 < 2"), "(< 1 2)");
    }

    #[test]
    fn parses_equality_expression() {
        assert_eq!(parse_and_print("1 == 1"), "(== 1 1)");
    }

    #[test]
    fn parses_equality_expression2() {
        assert_eq!(parse_and_print("1 == 1"), "(== 1 1)");
    }

    #[test]
    fn reports_error_on_unterminated_parentheses() {
        // A lone '(' cannot form a valid expression and should result in a ParseError.
        let mut scanner = Scanner::new("(".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_err());
    }
}
