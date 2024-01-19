use thiserror::Error; // https://docs.rs/thiserror/

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("[line {line}] Error: {msg}")]
    GenericError { line: usize, msg: String },

    #[error("[line {line}] Error: Unexpected character ('{c}')")]
    UnexpectedCharacter { line: usize, c: char },
}

impl Error {
    pub fn report(&self) {
        eprintln!("{self}")
    }
}
