use std::collections::HashMap;

use crate::ast::{Expr, Stmt, Value};
use crate::error::Error;
use crate::token::{Token, TokenValue};

pub struct Interpreter {
    env: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value, Vec<Error>> {
        let mut value = Value::Number(0.0);

        for statement in statements {
            match self.execute(statement) {
                Ok(v) => value = v,
                Err(e) => return Err(vec![e]),
            }
        }

        Ok(value)
    }

    fn execute(&mut self, stmt: Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Let(name, expr) => self.let_(name, expr),
            Stmt::Print(expr) => self.print(expr),
        }
    }

    fn let_(&mut self, name: Token, expr: Box<Expr>) -> Result<Value, Error> {
        if self.env.contains_key(&name.lexeme) {
            return Err(Error::new(
                name.line,
                name.lexeme,
                "Variable already defined.",
            ));
        }

        let val = self.evaluate(expr)?;
        self.env.insert(name.lexeme, val);
        Ok(Value::Null)
    }

    fn print(&mut self, expr: Box<Expr>) -> Result<Value, Error> {
        let result = self.evaluate(expr)?;
        println!("{}", result);
        Ok(Value::Null)
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<Value, Error> {
        match *expr {
            Expr::Assignment(name, expr) => self.assignment(name, expr),
            Expr::Binary(op, left, right) => self.binary(op, left, right),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value),
            Expr::Ternary(op, first, second, third) => self.ternary(op, first, second, third),
            Expr::Unary(op, right) => self.unary(op, right),
            Expr::Variable(name) => self.variable(name),
        }
    }

    fn assignment(&mut self, name: Token, expr: Box<Expr>) -> Result<Value, Error> {
        if !self.env.contains_key(&name.lexeme) {
            return Err(Error::new(name.line, name.lexeme, "Variable not defined."));
        }

        let val = self.evaluate(expr)?;
        self.env.insert(name.lexeme, val.clone());
        Ok(val)
    }

    fn binary(&mut self, op: Token, left: Box<Expr>, right: Box<Expr>) -> Result<Value, Error> {
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

    fn unary(&mut self, op: Token, right: Box<Expr>) -> Result<Value, Error> {
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
        &mut self,
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

    fn variable(&self, name: Token) -> Result<Value, Error> {
        match self.env.get(&name.lexeme) {
            Some(Value::Undefined) => Err(Error::new(
                name.line,
                name.lexeme,
                "Variable not initialized.",
            )),
            Some(val) => Ok(val.clone()),
            _ => Err(Error::new(name.line, name.lexeme, "Variable not defined.")),
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Number(n) => n != 0.0,
            Value::Null | Value::Undefined => false,
        }
    }
}
