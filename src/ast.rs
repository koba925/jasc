// TODO: ValueをTokenの定義でも使う

use std::fmt::Display;

use crate::token::Token;

fn vec_to_str<T: Display>(v: &Vec<T>) -> String {
    let mut first = true;
    let mut s = "(".to_string();
    for e in v {
        if first {
            first = false;
        } else {
            s.push_str(" ");
        }
        s.push_str(&format!("{}", e));
    }
    s.push_str(")");
    s
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Function(Vec<Token>, Vec<Stmt>),
    Number(f64),
    Null,
    Undefined,
}

// printで出力するフォーマット
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Function(parameters, _) => {
                let names = parameters.iter().map(|p| &p.lexeme).collect();
                write!(f, "(function {})", vec_to_str(&names))
            }
            Value::Number(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

// TODO: 全部にTokenを持たせる（Runtime Errorを出すときに必要）
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Call(Token, Box<Expr>, Vec<Expr>),
    Function(Vec<Token>, Vec<Stmt>),
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
            Expr::Call(_token, callee, args) => {
                write!(f, "(call {} {}", callee, vec_to_str(args))
            }
            Expr::Function(parameters, statements) => {
                write!(f, "(function (parameters")?;
                for parameter in parameters {
                    write!(f, " {}", parameter.lexeme)?;
                }
                write!(f, ") (statements")?;
                for statement in statements {
                    write!(f, " {}", statement)?;
                }
                write!(f, "))")
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

#[derive(Debug, PartialEq, Clone)]
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
