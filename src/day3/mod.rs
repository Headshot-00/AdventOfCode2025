use std::fmt;

pub mod accumulator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    EmptyInput,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::EmptyInput => write!(f, "Input was empty"),
        }
    }
}

impl std::error::Error for UpdateError {}
