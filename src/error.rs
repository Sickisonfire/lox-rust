use std::fmt;

pub type LoxResult<T> = std::result::Result<T, LoxError>;

#[derive(Debug, Clone)]
pub enum LoxError {
    SyntaxError { line: usize, message: String },
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error text goes here")
    }
}

impl std::error::Error for LoxError {}

