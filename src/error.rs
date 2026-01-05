/// Common Akahu error types as per the documentation.
///
/// [<https://developers.akahu.nz/docs/response-formatting#common-error-messages>]
#[derive(Debug, thiserror::Error)]
pub enum AkahuError {
    // API-level errors (from Akahu responses)
    /// Bad request - invalid request parameters
    #[error("Bad request: {message}")]
    BadRequest { message: String, status: u16 },

    /// Unauthorized - invalid or revoked authentication credentials
    #[error("Unauthorized: {message}")]
    Unauthorized { message: String },

    /// Forbidden - insufficient permissions or missing required headers
    #[error("Forbidden: {message}")]
    Forbidden { message: String },

    /// Not found - resource doesn't exist or is inaccessible
    #[error("Not found: {message}")]
    NotFound { message: String },

    /// Rate limited - too many requests
    #[error("Rate limited: {message}")]
    RateLimited { message: String },

    /// Internal server error - system-level failure
    #[error("Internal server error: {message}")]
    InternalServerError { message: String },

    /// Generic API error with status code and message
    #[error("API error {status}: {message}")]
    ApiError { status: u16, message: String },

    // Client-level errors
    /// Network error from reqwest
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Invalid header value
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// JSON deserialization error
    #[error("JSON deserialization error: {error}{}", .source_string.as_ref().map(|s| format!(" - {}", s)).unwrap_or_default())]
    JsonDeserialization {
        /// The deserialisation error that was generated.
        error: serde_json::Error,
        /// The string that was originally being deserialized, if available.
        source_string: Option<String>,
    },

    /// Missing app secret - call with_app_secret() first for app-scoped endpoints
    #[error("Missing app secret - call with_app_secret() first")]
    MissingAppSecret,

    // OAuth-specific errors
    /// OAuth error response (follows OAuth2 spec)
    #[error("OAuth error: {error}{}", .error_description.as_ref().map(|d| format!(" - {}", d)).unwrap_or_default())]
    OAuth {
        error: String,
        error_description: Option<String>,
    },
}

/// Convenience type alias for Results using AkahuError
pub(crate) type AkahuResult<T> = std::result::Result<T, AkahuError>;
