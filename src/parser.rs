use crate::scanner::Token;

#[derive(Debug)]
pub enum ExprNode {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl ExprNode {
    pub fn accept(&self, visitor: &mut dyn ExprVisitor) -> String {
        match self {
            ExprNode::Binary(expr) => visitor.visit_binary_expr(expr),
            ExprNode::Unary(expr) => visitor.visit_unary_expr(expr),
            ExprNode::Grouping(expr) => visitor.visit_grouping_expr(expr),
            ExprNode::Literal(expr) => visitor.visit_literal_expr(expr),
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
    pub left: Box<ExprNode>,
    pub operator: Token,
    pub right: Box<ExprNode>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprNode>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<ExprNode>,
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
