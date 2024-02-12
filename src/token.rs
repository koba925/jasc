#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenValue {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,
    Number(f64),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub val: TokenValue,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(val: TokenValue, lexeme: String, line: usize) -> Token {
        Token { val, lexeme, line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lexeme.fmt(f)
    }
}
