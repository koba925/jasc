// TODO: ValueをTokenの定義でも使う

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, op, right) => {
                f.write_fmt(format_args!("({} {} {})", op, left, right))
            }
            Expr::Grouping(expr) => f.write_fmt(format_args!("(group {})", expr)),
            Expr::Literal(val) => match val {
                Value::Number(n) => n.fmt(f),
            },
            Expr::Unary(op, right) => f.write_fmt(format_args!("({} {})", op, right)),
        }
    }
}
