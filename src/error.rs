use thiserror::Error; // https://docs.rs/thiserror/

#[derive(Error, Debug)]
pub enum Error {
    #[error("{msg}")]
    GenericError { msg: String },

    #[error("Unexpected character ('{c}')")]
    UnexpectedCharacter { c: char },
}

impl Error {
    pub fn report(&self) {
        eprintln!("Error: {self}")
    }
}
