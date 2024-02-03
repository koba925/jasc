use crate::ast::{Expr, Value};
use crate::error::Error;
use crate::token::{Token, TokenValue};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Box<Expr>) -> Result<Value, Vec<Error>> {
        match self.evaluate(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(vec![e]),
        }
    }

    fn evaluate(&self, expr: Box<Expr>) -> Result<Value, Error> {
        match *expr {
            Expr::Literal(value) => Ok(value),
            Expr::Binary(left, op, right) => self.binary(left, op, right),
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
