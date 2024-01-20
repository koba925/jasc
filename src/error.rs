#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub msg: String,
}

impl Error {
    pub fn new(line: usize, msg: String) -> Error {
        Error { line, msg }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.msg)
    }
}
