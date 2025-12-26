use std::fmt;

#[derive(Debug)]
pub enum UpdateError {
    EmptyInput,
    InvalidInput(String),
    Io(std::io::Error),
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::EmptyInput => write!(f, "Input was empty"),
            UpdateError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            UpdateError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for UpdateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UpdateError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for UpdateError {
    fn from(err: std::io::Error) -> Self {
        UpdateError::Io(err)
    }
}
