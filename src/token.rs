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

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => f.write_str("("),
            Self::RightParen => f.write_str(")"),
            Self::Plus => f.write_str("+"),
            Self::Minus => f.write_str("-"),
            Self::Star => f.write_str("*"),
            Self::Slash => f.write_str("/"),
            Self::Semicolon => f.write_str(";"),
            Self::Number(n) => n.fmt(f),
            Self::EOF => f.write_str("end"),
        }
    }
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
