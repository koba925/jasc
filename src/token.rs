#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenValue {
    Plus,
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
