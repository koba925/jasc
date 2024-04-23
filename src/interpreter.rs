use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, Stmt, Value};
use crate::env::Environment;
use crate::error::Error;
use crate::token::{Token, TokenValue};

#[derive(Debug, PartialEq)]
pub enum Runtime {
    Break(Value),
    Error(Error),
    Return(Value),
}

impl Runtime {
    fn from_token(token: &Token, msg: &str) -> Runtime {
        Runtime::Error(Error::from_token(token, msg))
    }
}

type Result<T, R = Runtime> = std::result::Result<T, R>;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Rc::new(RefCell::new(Default::default())),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value, Vec<Error>> {
        let mut value = Value::Null;

        for ref statement in statements {
            match self.execute(statement) {
                Ok(v) => value = v,
                Err(e) => {
                    return Err(vec![match e {
                        Runtime::Error(e) => e,
                        _ => unreachable!(),
                    }])
                }
            }
        }

        Ok(value)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Value> {
        match stmt {
            Stmt::Block(statements) => self.block(statements),
            Stmt::Break(expr) => self.break_(expr),
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::If(condition, consequence, alternative) => {
                self.if_(condition, consequence, alternative)
            }
            Stmt::Let(name, expr) => self.let_(name, expr),
            Stmt::Print(expr) => self.print(expr),
            Stmt::Return(expr) => self.return_(expr),
            Stmt::While(condition, statement) => self.while_(condition, statement),
        }
    }

    // TODO: callとコードを共有できないか
    fn block(&mut self, statements: &Vec<Stmt>) -> Result<Value> {
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

    fn break_(&mut self, expr: &Option<Box<Expr>>) -> Result<Value> {
        let mut val = Value::Null;
        if let Some(expr) = expr {
            val = self.evaluate(expr)?;
        }
        Err(Runtime::Break(val))
    }

    fn if_(
        &mut self,
        condition: &Expr,
        consequence: &Stmt,
        alternative: &Option<Box<Stmt>>,
    ) -> Result<Value> {
        let cond = self.evaluate(condition)?;
        if Self::is_truthy(&cond) {
            Ok(self.execute(consequence)?)
        } else if let Some(alt) = alternative {
            Ok(self.execute(alt)?)
        } else {
            Ok(Value::Null)
        }
    }

    fn let_(&mut self, name: &Token, expr: &Expr) -> Result<Value> {
        let val = self.evaluate(expr)?;
        self.env
            .borrow_mut()
            .define(name, val)
            .map_err(Runtime::Error)
    }

    fn print(&mut self, expr: &Expr) -> Result<Value> {
        let result = self.evaluate(expr)?;
        println!("{}", result);
        Ok(Value::Null)
    }

    // TODO: トップレベルでreturnしたら値を返して正常終了
    fn return_(&mut self, expr: &Option<Box<Expr>>) -> Result<Value> {
        let mut val = Value::Null;
        if let Some(expr) = expr {
            val = self.evaluate(expr)?;
        }
        Err(Runtime::Return(val))
    }

    fn while_(&mut self, condition: &Expr, statement: &Stmt) -> Result<Value> {
        let mut result = Ok(Value::Null);
        while Self::is_truthy(&self.evaluate(condition)?) {
            result = self.execute(&statement);
            if let Err(Runtime::Break(val)) = result {
                result = Ok(val);
                break;
            } else if let Err(_) = result {
                break;
            }
        }
        result
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Assignment(name, expr) => self.assignment(name, expr),
            Expr::Binary(op, left, right) => self.binary(op, left, right),
            Expr::Call(token, callee, args) => self.call(token, callee, args),
            Expr::Function(parameters, statements) => self.function(parameters, statements),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Ternary(op, first, second, third) => self.ternary(op, first, second, third),
            Expr::Unary(op, right) => self.unary(op, right),
            Expr::Variable(name) => self.variable(name),
        }
    }

    fn assignment(&mut self, name: &Token, expr: &Expr) -> Result<Value> {
        let val = self.evaluate(expr)?;
        self.env
            .borrow_mut()
            .assign(name, val)
            .map_err(Runtime::Error)
    }

    fn binary(&mut self, op: &Token, left: &Expr, right: &Expr) -> Result<Value> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Plus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                _ => Err(Runtime::from_token(op, "Operands must be two numbers.")),
            },
            TokenValue::Minus => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(Runtime::from_token(op, "Operands must be two numbers.")),
            },
            TokenValue::Star => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(Runtime::from_token(op, "Operands must be two numbers.")),
            },
            TokenValue::Slash => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => Err(Runtime::from_token(op, "Operands must be two numbers.")),
            },
            _ => Err(Runtime::from_token(op, "Unknown operation.")),
        }
    }

    fn call(&mut self, token: &Token, callee: &Expr, args: &Vec<Expr>) -> Result<Value> {
        let func = self.evaluate(callee)?;
        let Value::Function(parameters, statements, env) = func else {
            return Err(Runtime::from_token(token, "Callee is not a function."));
        };
        if parameters.len() != args.len() {
            return Err(Runtime::from_token(
                token,
                "Number of the arguments does not match.",
            ));
        }

        let closure = Environment::enclosed_by(&env);

        for (p, a) in parameters.iter().zip(args) {
            let val = self.evaluate(a)?;
            closure
                .borrow_mut()
                .define(p, val)
                .map_err(Runtime::Error)?;
        }

        let previous = Rc::clone(&self.env);
        self.env = Environment::enclosed_by(&closure);

        let mut result = Ok(Value::Null);

        for statement in statements {
            result = self.execute(&statement);
            if let Err(Runtime::Return(v)) = result {
                result = Ok(v);
                break;
            } else if let Err(_) = result {
                break;
            }
        }

        self.env = previous;
        result
    }

    fn unary(&mut self, op: &Token, right: &Expr) -> Result<Value> {
        let right_val = self.evaluate(right)?;

        match op.val {
            TokenValue::Minus => match right_val {
                Value::Number(r) => Ok(Value::Number(-r)),
                _ => Err(Runtime::from_token(op, "Operand must be a number.")),
            },
            _ => Err(Runtime::from_token(op, "Unknown operation.")),
        }
    }

    fn ternary(&mut self, op: &Token, first: &Expr, second: &Expr, third: &Expr) -> Result<Value> {
        assert_eq!(op.val, TokenValue::Question);

        let condition = self.evaluate(first)?;
        if Self::is_truthy(&condition) {
            self.evaluate(second)
        } else {
            self.evaluate(third)
        }
    }

    fn variable(&self, name: &Token) -> Result<Value> {
        self.env.borrow().get(name).map_err(Runtime::Error)
    }

    fn function(&mut self, parameters: &[Token], statements: &[Stmt]) -> Result<Value> {
        Ok(Value::Function(
            parameters.to_owned(),
            statements.to_owned(),
            Rc::clone(&self.env),
        ))
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Number(n) => n != &0.0,
            Value::Null | Value::Undefined => false,
            _ => true,
        }
    }
}
