use crate::parser::{Binary, ExprVisitor, Grouping, Literal, Unary};
use crate::token::{LiteralValue, Token, TokenType};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
}

impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        format!(
            "{} {} {}",
            expr.left.accept(self),
            expr.operator,
            expr.right.accept(self)
        )
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        format!("{}{}", expr.operator, expr.right.accept(self))
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        format!("({})", expr.expr.accept(self))
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        match &expr.value {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expr;
    use crate::token::{Token, TokenType};
    #[test]
    fn test_literal_number() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Number(123.45),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "123.45");
    }

    #[test]
    fn test_literal_string() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Literal(Literal {
            value: LiteralValue::String("hello".to_string()),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_literal_boolean() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Boolean(true),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "true");
    }

    #[test]
    fn test_literal_nil() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Nil,
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "nil");
    }

    #[test]
    fn test_unary_expression() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Unary(Unary {
            operator: Token::simple(TokenType::Minus, "-", 0),
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(123.0),
            })),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "-123");
    }

    #[test]
    fn test_binary_expression() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(1.0),
            })),
            operator: Token::simple(TokenType::Plus, "+", 0),
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(2.0),
            })),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "1 + 2");
    }

    #[test]
    fn test_grouping_expression() {
        let mut printer = AstPrinter::new();
        let expr = Expr::Grouping(Grouping {
            expr: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(45.67),
            })),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "(45.67)");
    }

    #[test]
    fn test_complex_expression() {
        let mut printer = AstPrinter::new();
        // Represents: (- (group (+ 1 2)))
        let expr = Expr::Unary(Unary {
            operator: Token::simple(TokenType::Minus, "-", 0),
            right: Box::new(Expr::Grouping(Grouping {
                expr: Box::new(Expr::Binary(Binary {
                    left: Box::new(Expr::Literal(Literal {
                        value: LiteralValue::Number(1.0),
                    })),
                    operator: Token::simple(TokenType::Plus, "+", 0),
                    right: Box::new(Expr::Literal(Literal {
                        value: LiteralValue::Number(2.0),
                    })),
                })),
            })),
        });

        let result = expr.accept(&mut printer);
        assert_eq!(result, "-(1 + 2)");
    }
}
