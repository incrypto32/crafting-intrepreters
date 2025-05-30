use crate::scanner::Token;

pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String;
    fn visit_unary_expr(&mut self, expr: &Unary) -> String;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String;
    fn visit_literal_expr(&mut self, expr: &Literal) -> String;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> String;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        visitor.visit_binary_expr(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        visitor.visit_unary_expr(self)
    }
}

pub struct Grouping {
    pub expr: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        visitor.visit_grouping_expr(self)
    }
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Literal {
    pub value: LiteralValue,
}

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        visitor.visit_literal_expr(self)
    }
}
