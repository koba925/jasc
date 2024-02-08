// TODO:matchesを作る → 必要か？

use crate::ast::{Expr, Value};
use crate::error::Error;
use crate::token::{Token, TokenValue};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, Vec<Error>> {
        let mut expr = Expr::Literal(Value::Number(0.0));
        let mut errors = vec![];

        match self.expression() {
            Ok(value) => expr = value,
            Err(error) => errors.push(error),
        }
        match self.consume(TokenValue::Semicolon, "Semicolon expected.") {
            Err(error) => errors.push(error),
            _ => (),
        };
        if errors.is_empty() {
            Ok(expr)
        } else {
            Err(errors)
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr> {
        let mut left = self.factor()?;

        loop {
            match self.peek().val {
                TokenValue::Plus | TokenValue::Minus => {
                    let op = self.advance().clone();
                    let right = self.factor()?;
                    left = Expr::Binary(Box::new(left), op, Box::new(right))
                }
                _ => return Ok(left),
            }
        }
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut left = self.unary()?;

        loop {
            match self.peek().val {
                TokenValue::Star | TokenValue::Slash => {
                    let op = self.advance().clone();
                    let right = self.unary()?;
                    left = Expr::Binary(Box::new(left), op, Box::new(right))
                }
                _ => return Ok(left),
            }
        }
    }

    fn unary(&mut self) -> Result<Expr> {
        match self.peek().val {
            TokenValue::Minus => {
                let op = self.advance().clone();
                let right = self.primary()?;
                Ok(Expr::Unary(op, Box::new(right)))
            }
            _ => return self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.advance().val {
            TokenValue::Number(value) => Ok(Expr::Literal(Value::Number(value))),
            _ => Err(Error::from_token(self.peek(), "Number expected.")),
        }
    }

    fn consume(&mut self, expected: TokenValue, msg: impl Into<String>) -> Result<&Token> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(Error::from_token(self.peek(), msg))
        }
    }

    fn check(&self, expected: TokenValue) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().val == expected
        }
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().val == TokenValue::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
