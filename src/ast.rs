// TODO: ValueをTokenの定義でも使う

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::env::Environment;
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
    Function(Vec<Token>, Vec<Stmt>, Rc<RefCell<Environment>>),
    Number(f64),
    Null,
    Undefined,
}

// printで出力するフォーマット
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Function(parameters, _, _) => {
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

// 全部 {:?} でもいいか？テストはどう書ける？
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

// TODO: 全部にTokenを持たせる？
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Let(Token, Box<Expr>),
    Print(Box<Expr>),
    Return(Option<Box<Expr>>),
    While(Box<Expr>, Box<Stmt>),
}

// 全部 {:?} でもいいか？テストはどう書ける？
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(statements) => {
                write!(f, "(block {})", stringify_statements(statements))
            }
            Stmt::Expression(expr) => write!(f, "(expression {})", expr),
            Stmt::If(condition, consequence, alternative) => {
                write!(
                    f,
                    "(if {} {}{})",
                    condition,
                    consequence,
                    if let Some(alt) = alternative {
                        format!(" {}", alt.to_string())
                    } else {
                        "".to_string()
                    }
                )
            }
            Stmt::Let(name, expr) => {
                write!(f, "(let {} {})", name.lexeme, expr)
            }
            Stmt::Print(expr) => {
                write!(f, "(print {})", expr)
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    write!(f, "(return {})", expr)
                } else {
                    write!(f, "(return)")
                }
            }
            Stmt::While(condition, statement) => {
                write!(f, "(while {} {})", condition, statement)
            }
        }
    }
}

pub fn stringify_statements(statements: &Vec<Stmt>) -> String {
    let mut result = String::new();
    let mut first = true;

    for statement in statements {
        if first {
            result = format!("{}", statement);
            first = false;
        } else {
            result = format!("{} {}", result, statement)
        }
    }

    result
}
