use crate::ast::{Expr, Stmt, Value};
use crate::error::Error;
use crate::token::{Token, TokenValue};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<Value, Vec<Error>> {
        let mut errors = vec![];
        let mut value = Value::Number(0.0);

        for statement in statements {
            match self.execute(statement) {
                Ok(v) => value = v,
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(value)
        } else {
            Err(errors)
        }
    }

    fn execute(&self, stmt: Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
        }
    }

    fn evaluate(&self, expr: Box<Expr>) -> Result<Value, Error> {
        match *expr {
            Expr::Binary(left, op, right) => self.binary(left, op, right),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value),
            Expr::Unary(op, right) => self.unary(op, right),
        }
    }

    fn binary(&self, left: Box<Expr>, op: Token, right: Box<Expr>) -> Result<Value, Error> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Plus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            },
            TokenValue::Minus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            },
            TokenValue::Star => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            },
            TokenValue::Slash => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            },
            _ => Err(Error::from_token(&op, "Unknown operation.")),
        }
    }

    fn unary(&self, op: Token, right: Box<Expr>) -> Result<Value, Error> {
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Minus => match right_val {
                Value::Number(r) => Ok(Value::Number(-r)),
            },
            _ => Err(Error::from_token(&op, "Unknown operation.")),
        }
    }
}
