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
            Stmt::Print(expr) => self.print(expr),
        }
    }

    fn print(&self, expr: Box<Expr>) -> Result<Value, Error> {
        let result = self.evaluate(expr)?;
        println!("{}", result);
        Ok(Value::Null)
    }

    fn evaluate(&self, expr: Box<Expr>) -> Result<Value, Error> {
        match *expr {
            Expr::Binary(op, left, right) => self.binary(op, left, right),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value),
            Expr::Ternary(op, first, second, third) => self.ternary(op, first, second, third),
            Expr::Unary(op, right) => self.unary(op, right),
        }
    }

    fn binary(&self, op: Token, left: Box<Expr>, right: Box<Expr>) -> Result<Value, Error> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Plus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                _ => Err(Error::from_token(&op, "Operands must be two numbers.")),
            },
            TokenValue::Minus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(Error::from_token(&op, "Operands must be two numbers.")),
            },
            TokenValue::Star => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(Error::from_token(&op, "Operands must be two numbers.")),
            },
            TokenValue::Slash => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => Err(Error::from_token(&op, "Operands must be two numbers.")),
            },
            _ => Err(Error::from_token(&op, "Unknown operation.")),
        }
    }

    fn unary(&self, op: Token, right: Box<Expr>) -> Result<Value, Error> {
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Minus => match right_val {
                Value::Number(r) => Ok(Value::Number(-r)),
                _ => Err(Error::from_token(&op, "Operand must be a number.")),
            },
            _ => Err(Error::from_token(&op, "Unknown operation.")),
        }
    }

    fn ternary(
        &self,
        op: Token,
        first: Box<Expr>,
        second: Box<Expr>,
        third: Box<Expr>,
    ) -> Result<Value, Error> {
        assert_eq!(op.val, TokenValue::Question);

        let condition = self.evaluate(first)?;
        if self.is_truthy(condition) {
            self.evaluate(second)
        } else {
            self.evaluate(third)
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Number(n) => n != 0.0,
            Value::Null => false,
        }
    }
}
