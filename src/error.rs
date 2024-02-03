use crate::token::{Token, TokenValue};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: usize,
    pub location: String,
    pub msg: String,
}

impl Error {
    pub fn new(line: usize, location: impl Into<String>, msg: impl Into<String>) -> Error {
        Error {
            line,
            location: location.into(),
            msg: msg.into(),
        }
    }

    pub fn from_token(token: &Token, msg: impl Into<String>) -> Error {
        match token.val {
            TokenValue::EOF => Self::new(token.line, "end", msg),
            _ => Self::new(token.line, token.lexeme.clone(), msg),
        }
    }

    pub fn report(&self) {
        if self.location == "" {
            eprintln!("[line {}] Error: {}", self.line, self.msg)
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                self.line, self.location, self.msg
            )
        }
    }
}
