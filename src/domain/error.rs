use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Missing API key: TOKEN_RAPIDAPI environment variable not set")]
    MissingApiKey,

    #[error("API error (HTTP {status}): {body}")]
    ApiError { status: u16, body: String },

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}
