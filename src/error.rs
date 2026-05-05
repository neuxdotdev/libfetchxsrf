use thiserror::Error;
#[derive(Error, Debug)]
pub enum XsrfError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("CSRF token not found for field: '{field}'")]
    TokenNotFound { field: String },
    #[error("Login failed with status: {status}")]
    LoginFailed { status: reqwest::StatusCode },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid response body")]
    InvalidBody,
}
pub type Result<T> = std::result::Result<T, XsrfError>;
