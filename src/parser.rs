use crate::scanner::Token;

pub trait ExprVisitor {}

pub trait Expr {
    fn visit(visitor: impl ExprVisitor);
}

pub struct Binary<T: Expr, U: Expr> {
    left: T,
    operator: Token,
    right: U,
}

pub struct Grouping<T: Expr> {
    expr: T,
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}

pub struct Literal {
    value: LiteralValue
}
