use crate::scanner::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl Expr {
    pub fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
        }
    }
}

pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String;
    fn visit_unary_expr(&mut self, expr: &Unary) -> String;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String;
    fn visit_literal_expr(&mut self, expr: &Literal) -> String;
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
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Literal {
    pub value: LiteralValue,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<_>) -> Self {
        Parser { tokens, current: 0 }
    }
}
