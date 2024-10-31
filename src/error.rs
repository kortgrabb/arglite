use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ArgLiteError {
    UnknownFlag(String),
    MissingArgument(String),
    ParseError(String),
}

impl fmt::Display for ArgLiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgLiteError::UnknownFlag(flag) => write!(f, "Unknown flag: {}", flag),
            ArgLiteError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            ArgLiteError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for ArgLiteError {}
