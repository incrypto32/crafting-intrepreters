use std::collections::HashMap;

use crate::parser::{
    Binary, Expr, ExprVisitorMut, Grouping, Literal, Stmt, StmtVisitorMut, Unary, VarAssignment,
};
use crate::token::{LiteralValue, Token, TokenType};

pub struct RuntimeError {
    pub message: String,
    pub line: usize,
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
            line: token.line,
        }
    }

    fn invalid_operator(token_type: TokenType, token: Token) -> Self {
        RuntimeError {
            message: format!("Invalid operator: {:?}", token_type),
            line: token.line,
        }
    }

    fn undefined_variable(name: String, line: usize) -> Self {
        RuntimeError {
            message: format!("Undefined variable: {}", name),
            line,
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

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

pub struct Interpreter {
    environment: Environment,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn get(&self, name: &String) -> Option<LiteralValue> {
        self.values.get(name).cloned()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            stmt.accept_mut(self)?;
        }
        Ok(())
    }

    pub fn define(&mut self, name: &String, value: LiteralValue) {
        self.environment.values.insert(name.clone(), value);
    }

    pub fn get(&self, line: usize, name: &String) -> Result<LiteralValue, RuntimeError> {
        self.environment
            .get(name)
            .ok_or_else(|| RuntimeError::undefined_variable(name.clone(), line))
    }
}

type LiteralValueResult = Result<LiteralValue, RuntimeError>;

impl StmtVisitorMut<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        expr.accept_mut::<LiteralValueResult>(self).map(|_| ())
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = expr.accept_mut::<LiteralValueResult>(self)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_variable(&mut self, var: &VarAssignment) -> Result<(), RuntimeError> {
        if let Some(expr) = &var.initializer {
            let value = expr.accept_mut::<LiteralValueResult>(self)?;
            let name = &var.token.lexeme;
            self.define(name, value);
            return Ok(());
        }

        self.define(&var.token.lexeme, LiteralValue::Nil);

        Ok(())
    }
}

impl ExprVisitorMut<Result<LiteralValue, RuntimeError>> for Interpreter {
    fn visit_binary(&mut self, expr: &Binary) -> Result<LiteralValue, RuntimeError> {
        let left = expr.left.accept_mut(self)?;
        let right = expr.right.accept_mut(self)?;
        evaluate_binary_expr(left, right, &expr.operator)
    }

    fn visit_variable(&mut self, token: &Token) -> Result<LiteralValue, RuntimeError> {
        self.get(token.line, &token.lexeme)
    }

    fn visit_assign(
        &mut self,
        token: &Token,
        value: &Box<Expr>,
    ) -> Result<LiteralValue, RuntimeError> {
        let val = value.accept_mut(self)?;
        self.define(&token.lexeme, val.clone());

        Ok(val)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<LiteralValue, RuntimeError> {
        let operator = expr.operator.typ;
        let right = expr.right.accept_mut(self)?;
        match (&operator, &right) {
            (TokenType::Minus, LiteralValue::Number(right)) => Ok(LiteralValue::Number(-right)),
            (TokenType::Bang, right) => Ok(LiteralValue::Boolean(!right.is_truthy())),
            _ => Err(RuntimeError::invalid_operator(
                operator,
                expr.operator.clone(),
            )),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<LiteralValue, RuntimeError> {
        expr.expr.accept_mut(self)
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<LiteralValue, RuntimeError> {
        Ok(expr.value.clone())
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
