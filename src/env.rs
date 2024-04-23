use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Value;
use crate::error::Error;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Environment {
    vars: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn enclosed_by(enclosing: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            vars: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }))
    }

    pub fn define(&mut self, name: &Token, val: Value) -> Result<Value, Error> {
        if self.vars.contains_key(&name.lexeme) {
            return Err(Error::from_token(name, "Variable already defined."));
        }

        self.vars.insert(name.lexeme.clone(), val);
        Ok(Value::Null)
    }

    pub fn assign(&mut self, name: &Token, val: Value) -> Result<Value, Error> {
        if self.vars.contains_key(&name.lexeme) {
            self.vars.insert(name.lexeme.clone(), val.clone());
            Ok(val)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, val)
        } else {
            Err(Error::from_token(name, "Variable not defined."))
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, Error> {
        match self.vars.get(&name.lexeme) {
            Some(Value::Undefined) => Err(Error::from_token(name, "Variable not initialized.")),
            Some(val) => Ok(val.clone()),
            _ => {
                if let Some(enclosing) = &self.enclosing {
                    enclosing.borrow().get(name)
                } else {
                    Err(Error::from_token(name, "Variable not defined."))
                }
            }
        }
    }
}
