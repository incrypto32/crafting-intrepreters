use std::fmt::Display;

use crate::token::{LiteralValue, Token, TokenType};

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Variable(Var),
}

impl Stmt {
    #[allow(dead_code)]
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Variable(var) => visitor.visit_variable(var),
        }
    }

    pub fn accept_mut<T>(&self, visitor: &mut dyn StmtVisitorMut<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Variable(var) => visitor.visit_variable(var),
        }
    }
}

#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
}

pub trait StmtVisitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
    fn visit_print(&self, expr: &Expr) -> T;
    fn visit_variable(&self, var: &Var) -> T;
}

pub trait StmtVisitorMut<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_print(&mut self, expr: &Expr) -> T;
    fn visit_variable(&mut self, var: &Var) -> T;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    /// Parse a single expression.
    #[allow(dead_code)]
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.consume(
            TokenType::SemiColon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Variable(Var { name, initializer }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Print(expr))
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
        let expr = parser.parse_expr().expect("Parser returned an error");

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
