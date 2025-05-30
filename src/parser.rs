use crate::scanner::Token;

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
}

pub trait Expr {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}

pub struct Grouping {
    pub expr: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}
