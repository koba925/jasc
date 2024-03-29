// TODO:matchesを作る → 必要か？

use crate::ast::{Expr, Stmt, Value};
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn statement(&mut self) -> Result<Stmt> {
        let result = match self.peek().val {
            TokenValue::LeftBrace => self.block_statement(),
            TokenValue::Print => self.print_statement(),
            TokenValue::Let => self.let_statement(),
            _ => self.expression_statement(),
        };
        match result {
            Ok(stmt) => Ok(stmt),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let mut statements = vec![];
        while self.peek().val != TokenValue::RightBrace {
            if self.is_at_end() {
                return Err(Error::new(self.peek().line, "end", "No closing brace."));
            }
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => return Err(error),
            }
        }
        self.advance();
        Ok(Stmt::Block(statements))
    }

    fn let_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let var = self.ternary()?;
        let Expr::Variable(name) = var else {
            return Err(Error::new(self.peek().line, "let", "Variable expected."));
        };
        let mut expr = Expr::Literal(Value::Undefined);
        if self.peek().val == TokenValue::Equal {
            self.advance();
            expr = self.expression()?;
        }
        self.consume(TokenValue::Semicolon, "Initializer or semicolon expected.")?;
        Ok(Stmt::Let(name, Box::new(expr)))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let expr = self.expression()?;
        self.consume(TokenValue::Semicolon, "Semicolon expected.")?;
        Ok(Stmt::Print(Box::new(expr)))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenValue::Semicolon, "Semicolon expected.")?;
        Ok(Stmt::Expression(Box::new(expr)))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let var = self.ternary()?;
        let Expr::Variable(ref token) = var else {
            return Ok(var);
        };
        if self.peek().val != TokenValue::Equal {
            return Ok(var);
        };
        self.advance();
        let expr = self.assignment()?;
        Ok(Expr::Assignment(token.clone(), Box::new(expr)))
    }

    fn ternary(&mut self) -> Result<Expr> {
        let first = self.term()?;

        match self.peek().val {
            TokenValue::Question => {
                let op = self.advance().clone();
                let second = self.ternary()?;
                self.consume(TokenValue::Colon, "Colon expected.")?;
                let third = self.ternary()?;
                Ok(Expr::Ternary(
                    op,
                    Box::new(first),
                    Box::new(second),
                    Box::new(third),
                ))
            }
            _ => Ok(first),
        }
    }

    fn term(&mut self) -> Result<Expr> {
        let mut left = self.factor()?;

        loop {
            match self.peek().val {
                TokenValue::Plus | TokenValue::Minus => {
                    let op = self.advance().clone();
                    let right = self.factor()?;
                    left = Expr::Binary(op, Box::new(left), Box::new(right))
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
                    left = Expr::Binary(op, Box::new(left), Box::new(right))
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
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance();

        match &token.val {
            TokenValue::Number(n) => Ok(Expr::Literal(Value::Number(n.clone()))),
            TokenValue::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenValue::RightParen, "Right paren expected")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenValue::Identifier => Ok(Expr::Variable(token.clone())),
            _ => Err(Error::from_token(
                token,
                format!("Expression expected, found `{}`", token.val),
            )),
        }
    }

    // TODO:synchronizeの精度を高める
    fn synchronize(&mut self) {
        // self.advance();
        while !self.is_at_end() {
            if self.previous().val == TokenValue::Semicolon {
                return;
            }
            self.advance();
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
