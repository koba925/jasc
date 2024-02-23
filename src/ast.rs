// TODO: ValueをTokenの定義でも使う

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Ternary(Token, Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(op, left, right) => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(val) => write!(f, "{}", val),
            Expr::Ternary(op, first, second, third) => {
                write!(f, "({} {} {} {})", op.lexeme, first, second, third)
            }
            Expr::Unary(op, right) => write!(f, "({} {})", op.lexeme, right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "(expression {})", expr),
            Stmt::Print(expr) => write!(f, "(print {})", expr),
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
