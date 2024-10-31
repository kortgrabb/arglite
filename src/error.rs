#[derive(Debug)]
pub enum ArgLiteError {
    MissingArgument(String),
    UnknownFlag(String),
    ParseError(String),
}

impl std::fmt::Display for ArgLiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgLiteError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            ArgLiteError::UnknownFlag(flag) => write!(f, "Unknown flag: {}", flag),
            ArgLiteError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ArgLiteError {}
