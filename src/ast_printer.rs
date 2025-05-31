use crate::parser::{Binary, ExprVisitor, Grouping, Literal, LiteralValue, Unary};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&crate::parser::Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        result.push(')');
        result
    }
}

impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.to_string(), &[&expr.left, &expr.right])
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.to_string(), &[&expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expr])
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
