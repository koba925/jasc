// TODO: ValueをTokenの定義でも使う

use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    Null,
    Undefined,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Ternary(Token, Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assignment(name, expr) => {
                write!(f, "(assignment {} {})", name.lexeme, expr)
            }
            Expr::Binary(op, left, right) => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(val) => write!(f, "{}", val),
            Expr::Ternary(op, first, second, third) => {
                write!(f, "({} {} {} {})", op.lexeme, first, second, third)
            }
            Expr::Variable(name) => write!(f, "(var {})", name.lexeme),
            Expr::Unary(op, right) => write!(f, "({} {})", op.lexeme, right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    Let(Token, Box<Expr>),
    Print(Box<Expr>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(statements) => {
                write!(f, "(block")?;
                for statement in statements {
                    write!(f, " {}", statement)?;
                }
                write!(f, ")")
            }
            Stmt::Expression(expr) => write!(f, "(expression {})", expr),
            Stmt::Let(name, expr) => {
                write!(f, "(let {} {})", name.lexeme, expr)
            }
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
