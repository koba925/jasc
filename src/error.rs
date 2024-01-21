#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: usize,
    pub msg: String,
}

impl Error {
    pub fn new(line: usize, msg: impl Into<String>) -> Error {
        Error {
            line,
            msg: msg.into(),
        }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.msg)
    }
}
