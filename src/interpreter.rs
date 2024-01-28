use crate::ast::{Expr, Value};
use crate::error::Error;
use crate::token::TokenValue;

pub struct Interpreter {
    expr: Expr,
}

impl Interpreter {
    pub fn new(expr: Expr) -> Interpreter {
        Interpreter { expr }
    }

    pub fn interpret(self) -> Result<Value, Vec<Error>> {
        match self.expr {
            Expr::Literal(value) => Ok(value),
            // TODO::&とか*とかrefとか見直す
            Expr::Binary(left, op, right) => match op.val {
                TokenValue::Plus => match (*left, *right) {
                    (Expr::Literal(l), Expr::Literal(r)) => match (l, r) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    },
                    _ => Err(vec![Error::from_token(&op, "Operands must be numbers.")]),
                },
                TokenValue::Minus => match (*left, *right) {
                    (Expr::Literal(l), Expr::Literal(r)) => match (l, r) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    },
                    _ => Err(vec![Error::from_token(&op, "Operands must be numbers.")]),
                },
                _ => Err(vec![Error::from_token(&op, "Unknown operation.")]),
            },
        }
    }
}
