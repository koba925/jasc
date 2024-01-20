use crate::error::Error;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            src: src.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = vec![];
        let mut errors = vec![];

        self.skip_whitespaces();
        while !self.is_at_end() {
            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(error) => errors.push(error),
            }
            self.skip_whitespaces();
        }

        self.start = self.current;
        tokens.push(self.make_token(TokenValue::EOF));

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_token(&mut self) -> Result<Token, Error> {
        self.start = self.current;

        match self.advance() {
            ';' => Ok(self.make_token(TokenValue::Semicolon)),
            c if c.is_ascii_digit() => Ok(self.number()),
            c => Err(Error::new(
                self.line,
                format!("Unexpected character ('{}').", c),
            )),
        }
    }

    fn skip_whitespaces(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
    }

    fn number(&mut self) -> Token {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        self.make_token(TokenValue::Number(self.lexeme().parse().unwrap()))
    }

    fn make_token(&self, val: TokenValue) -> Token {
        Token::new(val, self.lexeme(), self.line)
    }

    // self.currentの手前の文字までを切り出すことに注意
    fn lexeme(&self) -> String {
        self.src[self.start..self.current].iter().collect()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.src[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn peek(&self) -> char {
        self.src[self.current]
    }
}
