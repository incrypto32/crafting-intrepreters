use crate::parser::{Binary, Expr, ExprVisitor, Grouping, Literal, Unary};
use crate::token::{LiteralValue, Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        expr.accept(self)
    }
}

pub struct RuntimeError {
    pub message: String,
    pub token: Token,
}

impl RuntimeError {
    fn invalid_operands(
        left: LiteralValue,
        right: LiteralValue,
        message: &str,
        token: Token,
    ) -> Self {
        RuntimeError {
            message: format!(
                "Invalid operands: {} and {}. {}",
                format_literal(&left),
                format_literal(&right),
                message
            ),
            token,
        }
    }

    fn invalid_operator(token_type: TokenType, token: Token) -> Self {
        RuntimeError {
            message: format!("Invalid operator: {:?}", token_type),
            token,
        }
    }
}

fn format_literal(literal: &LiteralValue) -> String {
    match literal {
        LiteralValue::Number(n) => n.to_string(),
        LiteralValue::String(s) => format!("\"{}\"", s),
        LiteralValue::Boolean(b) => b.to_string(),
        LiteralValue::Nil => "nil".to_string(),
    }
}

fn evaluate_binary_expr(
    left: LiteralValue,
    right: LiteralValue,
    op: &Token,
) -> Result<LiteralValue, RuntimeError> {
    let operator_type = op.typ;

    let num = |f: fn(f64, f64) -> f64| match (&left, &right) {
        (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(f(*l, *r))),
        _ => Err(RuntimeError::invalid_operands(
            left.clone(),
            right.clone(),
            "Expected numbers",
            op.clone(),
        )),
    };

    let eq = |f: fn(&LiteralValue, &LiteralValue) -> bool| -> Result<LiteralValue, RuntimeError> {
        match (&left, &right) {
            (LiteralValue::Number(_), LiteralValue::Number(_))
            | (LiteralValue::String(_), LiteralValue::String(_))
            | (LiteralValue::Boolean(_), LiteralValue::Boolean(_)) => {
                Ok(LiteralValue::Boolean(f(&left, &right)))
            }
            _ => Err(RuntimeError::invalid_operands(
                left.clone(),
                right.clone(),
                "Expected comparable types",
                op.clone(),
            )),
        }
    };

    match operator_type {
        TokenType::Plus => match (&left, &right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Number(l + r)),
            (LiteralValue::String(l), LiteralValue::String(r)) => {
                Ok(LiteralValue::String(format!("{}{}", l, r)))
            }
            (LiteralValue::Number(_), _) => Err(RuntimeError::invalid_operands(
                left,
                right,
                "Expected a number",
                op.clone(),
            )),
            (LiteralValue::String(_), _) => Err(RuntimeError::invalid_operands(
                left,
                right,
                "Expected a string",
                op.clone(),
            )),
            _ => Err(RuntimeError::invalid_operands(
                left,
                right,
                "Expected a number or string",
                op.clone(),
            )),
        },
        TokenType::Minus => num(|l, r| l - r),
        TokenType::Star => num(|l, r| l * r),
        TokenType::Slash => num(|l, r| l / r),
        TokenType::EqualEqual => eq(|l, r| l == r),
        TokenType::BangEqual => eq(|l, r| l != r),
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
            match (&left, &right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    let result = match operator_type {
                        TokenType::Greater => l > r,
                        TokenType::GreaterEqual => l >= r,
                        TokenType::Less => l < r,
                        TokenType::LessEqual => l <= r,
                        _ => unreachable!(),
                    };
                    Ok(LiteralValue::Boolean(result))
                }
                _ => Err(RuntimeError::invalid_operands(
                    left,
                    right,
                    "Expected numbers",
                    op.clone(),
                )),
            }
        }
        _ => Err(RuntimeError::invalid_operator(operator_type, op.clone())),
    }
}

impl ExprVisitor<Result<LiteralValue, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<LiteralValue, RuntimeError> {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;
        evaluate_binary_expr(left, right, &expr.operator)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<LiteralValue, RuntimeError> {
        let operator = expr.operator.typ;
        let right = expr.right.accept(self)?;
        match (&operator, &right) {
            (TokenType::Minus, LiteralValue::Number(right)) => Ok(LiteralValue::Number(-right)),
            (TokenType::Bang, right) => Ok(LiteralValue::Boolean(!right.is_truthy())),
            _ => Err(RuntimeError::invalid_operator(
                operator,
                expr.operator.clone(),
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LiteralValue, RuntimeError> {
        expr.expr.accept(self)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LiteralValue, RuntimeError> {
        Ok(expr.value.clone())
    }
}
