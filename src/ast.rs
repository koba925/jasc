use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Value),
}
