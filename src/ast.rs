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
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(val) => match val {
                Value::Number(n) => n.fmt(f),
            },
            Expr::Unary(op, right) => write!(f, "({} {})", op.lexeme, right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "(expression {})", expr),
        }
    }
}

pub fn stringify_statements(statements: &Vec<Stmt>) -> String {
    let mut result = String::new();

    for statement in statements {
        result = format!("{}{}\n", result, statement)
    }

    result
}
