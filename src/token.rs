#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenValue {
    Semicolon,
    Number(f64),
    EOF,
}

#[derive(Debug, PartialEq)]
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
