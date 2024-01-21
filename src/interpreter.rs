use crate::ast::{Expr, Value};
use crate::error::Error;

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
        }
    }
}
