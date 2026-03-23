use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    MissingApiKey,
    ApiError { status: u16, body: String },
    NetworkError(String),
    ParsingError(String),
    InvalidArgument(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingApiKey => write!(f, "Missing API key: TOKEN_RAPIDAPI environment variable not set"),
            AppError::ApiError { status, body } => write!(f, "API error (HTTP {}): {}", status, body),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
            AppError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
