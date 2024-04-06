use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, Stmt, Value};
use crate::env::Environment;
use crate::error::Error;
use crate::token::{Token, TokenValue};

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value, Vec<Error>> {
        let mut value = Value::Null;

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
            Stmt::Block(statements) => self.block(statements),
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Let(name, expr) => self.let_(name, expr),
            Stmt::Print(expr) => self.print(expr),
        }
    }

    fn block(&mut self, statements: Vec<Stmt>) -> Result<Value, Error> {
        let enclosing = Rc::clone(&self.env);
        self.env = Environment::enclosed_by(&self.env);

        let mut value = Value::Null;

        for statement in statements {
            match self.execute(statement) {
                Ok(v) => value = v,
                Err(e) => {
                    self.env = enclosing;
                    return Err(e);
                }
            }
        }

        self.env = enclosing;
        Ok(value)
    }

    fn let_(&mut self, name: Token, expr: Box<Expr>) -> Result<Value, Error> {
        let val = self.evaluate(expr)?;
        self.env.borrow_mut().define(&name, val)
    }

    fn print(&mut self, expr: Box<Expr>) -> Result<Value, Error> {
        let result = self.evaluate(expr)?;
        println!("{}", result);
        Ok(Value::Null)
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<Value, Error> {
        match *expr {
            Expr::Assignment(name, expr) => self.assignment(&name, expr),
            Expr::Binary(op, left, right) => self.binary(&op, left, right),
            Expr::Call(ref token, callee, args) => self.call(token, callee, args),
            Expr::Function(parameters, statements) => self.function(parameters, statements),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value),
            Expr::Ternary(op, first, second, third) => self.ternary(&op, first, second, third),
            Expr::Unary(op, right) => self.unary(&op, right),
            Expr::Variable(name) => self.variable(&name),
        }
    }

    fn assignment(&mut self, name: &Token, expr: Box<Expr>) -> Result<Value, Error> {
        let val = self.evaluate(expr)?;
        self.env.borrow_mut().assign(&name, val)
    }

    fn binary(&mut self, op: &Token, left: Box<Expr>, right: Box<Expr>) -> Result<Value, Error> {
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

    fn call(&mut self, token: &Token, callee: Box<Expr>, args: Vec<Expr>) -> Result<Value, Error> {
        let func = self.evaluate(callee)?;
        let Value::Function(parameters, statements) = func else {
            return Err(Error::from_token(token, "Callee is not a function."));
        };
        if parameters.len() != args.len() {
            return Err(Error::from_token(
                token,
                "Number of the arguments does not match.",
            ));
        }

        let enclosing = Rc::clone(&self.env);
        self.env = Environment::enclosed_by(&self.env);

        for (p, a) in parameters.iter().zip(args) {
            let val = self.evaluate(Box::new(a))?;
            self.env.borrow_mut().define(&p, val)?;
        }

        let mut value = Value::Null;

        for statement in statements {
            match self.execute(statement) {
                Ok(v) => value = v,
                Err(e) => {
                    self.env = enclosing;
                    return Err(e);
                }
            }
        }

        self.env = enclosing;
        Ok(value)
    }

    fn unary(&mut self, op: &Token, right: Box<Expr>) -> Result<Value, Error> {
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
        op: &Token,
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

    fn variable(&self, name: &Token) -> Result<Value, Error> {
        self.env.borrow().get(name)
    }

    fn function(&mut self, parameters: Vec<Token>, statements: Vec<Stmt>) -> Result<Value, Error> {
        Ok(Value::Function(parameters, statements))
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Number(n) => n != 0.0,
            Value::Null | Value::Undefined => false,
            _ => true,
        }
    }
}
