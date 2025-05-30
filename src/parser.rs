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

pub struct Literal {
    value: 
}
