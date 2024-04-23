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
        match self.peek().val {
            TokenValue::LeftBrace => self.block_statement(),
            TokenValue::Break => self.break_statement(),
            TokenValue::If => self.if_statement(),
            TokenValue::Let => self.let_statement(),
            TokenValue::Print => self.print_statement(),
            TokenValue::Return => self.return_statement(),
            TokenValue::While => self.while_statement(),
            _ => self.expression_statement(),
        }
        .map_err(|e| {
            self.synchronize();
            e
        })
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let statements = self.block()?;
        self.consume(TokenValue::RightBrace, "Right brace expected.")?;
        Ok(Stmt::Block(statements))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.check(TokenValue::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?)
        }
        Ok(statements)
    }

    fn break_statement(&mut self) -> Result<Stmt> {
        let mut expr = None;
        self.advance();
        if self.peek().val != TokenValue::Semicolon {
            expr = Some(Box::new(self.expression()?));
        }
        self.consume(TokenValue::Semicolon, "Semicolon expected.")?;
        Ok(Stmt::Break(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.advance();
        self.consume(TokenValue::LeftParen, "Left paren expected.")?;
        let condition = self.expression()?;
        self.consume(TokenValue::RightParen, "Right paren expected.")?;
        let consequence = self.statement()?;
        let mut alternative = None;
        if self.peek().val == TokenValue::Else {
            self.advance();
            alternative = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If(
            Box::new(condition),
            Box::new(consequence),
            alternative,
        ))
    }

    fn let_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let var = self.ternary()?;
        let Expr::Variable(name) = var else {
            return Err(Error::new(self.peek().line, "let", "Variable expected."));
        };
        let mut expr = Expr::Literal(Value::Undefined);
        if self.check(TokenValue::Equal) {
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

    fn return_statement(&mut self) -> Result<Stmt> {
        let mut expr = None;
        self.advance();
        if self.peek().val != TokenValue::Semicolon {
            expr = Some(Box::new(self.expression()?));
        }
        self.consume(TokenValue::Semicolon, "Semicolon expected.")?;
        Ok(Stmt::Return(expr))
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.advance();
        self.consume(TokenValue::LeftParen, "Left paren expected.")?;
        let condition = self.expression()?;
        self.consume(TokenValue::RightParen, "Right paren expected.")?;
        let statement = self.statement()?;
        Ok(Stmt::While(Box::new(condition), Box::new(statement)))
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
        if !self.check(TokenValue::Equal) {
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
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        while self.check(TokenValue::LeftParen) {
            let token = self.peek().clone();
            self.advance();
            let args = self.arguments()?;
            expr = Expr::Call(token, Box::new(expr), args);
        }
        Ok(expr)
    }

    fn arguments(&mut self) -> Result<Vec<Expr>> {
        let mut args = vec![];

        while !self.check(TokenValue::RightParen) {
            if self.is_at_end() {
                return Err(Error::from_token(self.peek(), "No closing parenthesis."));
            }
            let expr = self.expression()?;
            args.push(expr);
            if !self.check(TokenValue::RightParen) {
                self.consume(TokenValue::Comma, "Comma expected.")?;
            }
        }

        self.advance();
        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance();

        match &token.val {
            TokenValue::Number(n) => Ok(Expr::Literal(Value::Number(*n))),
            TokenValue::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenValue::RightParen, "Right paren expected")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenValue::Function => self.function(),
            TokenValue::Identifier => Ok(Expr::Variable(token.clone())),
            _ => Err(Error::from_token(
                token,
                &format!("Expression expected, found `{}`", token.val),
            )),
        }
    }

    fn function(&mut self) -> Result<Expr> {
        self.consume(TokenValue::LeftParen, "Left parenthesis expected")?;
        let parameters = self.parameters()?;
        self.consume(TokenValue::LeftBrace, "Left brace expected")?;
        let statements = self.block()?;
        self.consume(TokenValue::RightBrace, "Right brace expected.")?;

        Ok(Expr::Function(parameters, statements))
    }

    fn parameters(&mut self) -> Result<Vec<Token>> {
        let mut parameters = vec![];

        if !self.check(TokenValue::RightParen) {
            while !self.is_at_end() {
                parameters.push(
                    self.consume(TokenValue::Identifier, "Identifier expected.")?
                        .clone(),
                );
                if !self.check(TokenValue::Comma) {
                    break;
                }
                self.advance();
            }
        }
        self.consume(TokenValue::RightParen, "Right paren expected.")?;
        Ok(parameters)
    }

    fn synchronize(&mut self) {
        // self.advance();
        while !self.is_at_end() {
            if self.previous().val == TokenValue::Semicolon {
                return;
            }
            if let TokenValue::If | TokenValue::Let | TokenValue::Print = self.peek().val {
                return;
            }
            self.advance();
        }
    }

    fn consume(&mut self, expected: TokenValue, msg: &str) -> Result<&Token> {
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
