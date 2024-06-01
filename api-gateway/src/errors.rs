use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiGatewayError {

    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Failed to parse request body: {0}")]
    RequestBodyParseError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Authorization failed: {0}")]
    AuthorizationError(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Resource not found")]
    NotFound,
    #[error("Other error: {0}")]
    Other(String),
}

impl From<hyper::Error> for ApiGatewayError {
    fn from(err: hyper::Error) -> Self {
        ApiGatewayError::Other(err.to_string())
    }
}

impl From<std::io::Error> for ApiGatewayError {
    fn from(err: std::io::Error) -> Self {
        ApiGatewayError::Other(err.to_string())
    }
}

