use std::fmt;

pub mod accumulator;
pub mod digits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    EmptyInput,
    InvalidInput,
    ReversedRange,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::EmptyInput => write!(f, "Input was empty"),
            UpdateError::InvalidInput => write!(f, "Input was malformed"),
            UpdateError::ReversedRange => write!(f, "Range start is greater than range end"),
        }
    }
}

impl std::error::Error for UpdateError {}
