use crate::token::{Token, TokenValue};

// TODO: 文字の位置を覚える
#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: usize,
    pub location: String,
    pub msg: String,
}

impl Error {
    pub fn new(line: usize, location: &str, msg: &str) -> Error {
        Error {
            line,
            location: location.to_string(),
            msg: msg.to_string(),
        }
    }

    pub fn from_token(token: &Token, msg: &str) -> Error {
        match token.val {
            TokenValue::EOF => Self::new(token.line, "end", msg),
            _ => Self::new(token.line, &token.lexeme, msg),
        }
    }

    pub fn report(&self) {
        if self.location.is_empty() {
            eprintln!("[line {}] Error: {}", self.line, self.msg)
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                self.line,
                self.location.escape_debug(),
                self.msg
            )
        }
    }
}
