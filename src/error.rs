use std::fmt;

// TODO: refactor error handling to be more idiomatic
pub type LoxResult<T> = std::result::Result<T, LoxError>;

#[derive(Debug)]
pub enum LoxError {
    SyntaxError { line: usize, message: String },
    IoError(std::io::Error),
}

impl std::error::Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::SyntaxError { line, message } => {
                write!(f, "Error at line: {} -> {}", line, message)
            }
            LoxError::IoError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

// convert other errors so we can use them with LoxResult
impl From<std::io::Error> for LoxError {
    fn from(err: std::io::Error) -> Self {
        LoxError::IoError(err)
    }
}
