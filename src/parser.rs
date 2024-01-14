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

    pub fn parse(&mut self) -> Result<f64, Vec<Error>> {
        let mut expr = 0.0;
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

    fn expression(&mut self) -> Result<f64> {
        self.primary()
    }

    fn primary(&mut self) -> Result<f64> {
        match self.advance().val {
            TokenValue::Number(value) => Ok(value),
            _ => Err(Error::GenericError {
                line: self.peek().line,
                msg: "Number expected.".to_string(),
            }),
        }
    }

    fn consume<S: ToString>(&mut self, expected: TokenValue, msg: S) -> Result<&Token> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(Error::GenericError {
                line: self.peek().line,
                msg: msg.to_string(),
            })
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
