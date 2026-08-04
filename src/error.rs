/// The raw body of a response that could not be deserialised, kept for debugging.
///
/// This is data *from the bank*, not a message about it: a page of transactions carries
/// merchant names, amounts, balances and account numbers for a real person. Nothing in this
/// crate ever prints it —
///
/// - [`AkahuError::JsonDeserialization`]'s `Display` shows only the `serde_json` error,
///   which already carries the line and column the parse failed at (the diagnostic part);
/// - this type's `Debug` reports the length and nothing else, so a caller that logs the
///   error with `{:?}` — or an error wrapper that does it for them — cannot spill the body
///   by accident.
///
/// Read it with [`ResponseBody::as_str`] when you actually want it, and treat what comes
/// back as sensitive: it is not safe to log, persist, or return to an API client.
#[derive(Clone)]
pub struct ResponseBody(String);

impl ResponseBody {
    /// Wrap raw response bytes, replacing any invalid UTF-8 with the replacement character.
    ///
    /// JSON is UTF-8 by definition, so the lossy conversion only matters for a body that is
    /// truncated or wasn't JSON in the first place — exactly the cases this type exists for.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        Self(String::from_utf8_lossy(bytes).into_owned())
    }

    /// The body as a string. **Sensitive** — see the type-level note.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the body. **Sensitive** — see the type-level note.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// The length of the body in bytes. Safe to log.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the body was empty. Safe to log.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Debug for ResponseBody {
    /// Redacted deliberately — the length is diagnostic, the contents are not ours to print.
    /// See the type-level note.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseBody(<redacted, {} bytes>)", self.0.len())
    }
}

/// Common Akahu error types as per the documentation.
///
/// [<https://developers.akahu.nz/docs/response-formatting#common-error-messages>]
#[derive(Debug, thiserror::Error)]
pub enum AkahuError {
    // API-level errors (from Akahu responses)
    /// Bad request - invalid request parameters
    #[error("Bad request: {message}")]
    BadRequest {
        /// Error message from the API
        message: String,
        /// HTTP status code (400)
        status: u16,
    },

    /// Unauthorized - invalid or revoked authentication credentials
    #[error("Unauthorized: {message}")]
    Unauthorized {
        /// Error message from the API
        message: String,
    },

    /// Forbidden - insufficient permissions or missing required headers
    #[error("Forbidden: {message}")]
    Forbidden {
        /// Error message from the API
        message: String,
    },

    /// Not found - resource doesn't exist or is inaccessible
    #[error("Not found: {message}")]
    NotFound {
        /// Error message from the API
        message: String,
    },

    /// Rate limited - too many requests
    #[error("Rate limited: {message}")]
    RateLimited {
        /// Error message from the API
        message: String,
    },

    /// Internal server error - system-level failure
    #[error("Internal server error: {message}")]
    InternalServerError {
        /// Error message from the API
        message: String,
    },

    /// Generic API error with status code and message
    #[error("API error {status}: {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
    },

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
    ///
    /// The `Display` of this variant is only ever the `serde_json` error — which names the
    /// line and column it gave up at — never the body itself. See [`ResponseBody`].
    #[error("JSON deserialization error: {error}")]
    JsonDeserialization {
        /// The deserialisation error that was generated.
        error: serde_json::Error,
        /// The body that failed to deserialize, if it was read before the failure.
        ///
        /// **Sensitive**, and deliberately absent from this error's `Display` and redacted in
        /// its `Debug` — see [`ResponseBody`] for what it holds and why.
        response_body: Option<ResponseBody>,
    },

    /// The response body was larger than the client's configured ceiling.
    ///
    /// A request timeout bounds how *long* a response may take, not how many bytes it may
    /// be, so this is the only thing standing between a caller and an unbounded read. Raise
    /// the ceiling with [`AkahuClient::with_max_response_bytes`] if a legitimate response is
    /// hitting it.
    ///
    /// [`AkahuClient::with_max_response_bytes`]: crate::AkahuClient::with_max_response_bytes
    #[error("Response body too large: exceeds the {max_bytes} byte maximum{}", .declared_bytes.map(|n| format!(" (Content-Length declared {n} bytes)")).unwrap_or_default())]
    ResponseTooLarge {
        /// The ceiling that was exceeded, in bytes.
        max_bytes: u64,
        /// The size the response declared in its `Content-Length` header, if it declared one.
        /// Absent for a chunked or otherwise length-less response, which is detected part-way
        /// through the read instead.
        declared_bytes: Option<u64>,
    },

    /// Missing app secret - call with_app_secret() first for app-scoped endpoints
    #[error("Missing app secret - call with_app_secret() first")]
    MissingAppSecret,

    // OAuth-specific errors
    /// OAuth error response (follows OAuth2 spec)
    #[error("OAuth error: {error}{}", .error_description.as_ref().map(|d| format!(" - {}", d)).unwrap_or_default())]
    OAuth {
        /// OAuth error code (e.g., "invalid_grant")
        error: String,
        /// Optional human-readable error description
        error_description: Option<String>,
    },
}

/// Convenience type alias for Results using AkahuError
pub type AkahuResult<T> = std::result::Result<T, AkahuError>;

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "tests need to unwrap to verify correctness"
)]
mod tests {
    use super::*;

    /// A page of transactions that fails to parse must not end up in the error's text. A
    /// caller has no way to know where a `Display` lands — a log line, a database row, an
    /// HTTP error body — and the bank's answer is not ours to put there.
    #[test]
    fn deser_error_display_does_not_contain_the_response_body() {
        let body = r#"{"success":true,"items":[{"description":"FLAT WHITE THE ROASTERY","amount":-5.50}]}"#;
        let error = AkahuError::JsonDeserialization {
            error: serde_json::from_str::<u32>(body).unwrap_err(),
            response_body: Some(ResponseBody::from_bytes(body.as_bytes())),
        };

        let rendered = error.to_string();
        assert!(
            !rendered.contains("ROASTERY") && !rendered.contains("5.50"),
            "response body leaked into Display: {rendered}"
        );
        // What's left still has to be diagnostic: serde names the position it gave up at.
        assert!(
            rendered.contains("line 1"),
            "Display lost the serde position: {rendered}"
        );
    }

    /// The same holds for `Debug`, which is what most logging macros actually reach for
    /// (`tracing::error!(?err)`), and which a derived `Debug` on the enum would otherwise
    /// print in full.
    #[test]
    fn deser_error_debug_redacts_the_response_body() {
        let body = r#"{"description":"FLAT WHITE THE ROASTERY"}"#;
        let error = AkahuError::JsonDeserialization {
            error: serde_json::from_str::<u32>(body).unwrap_err(),
            response_body: Some(ResponseBody::from_bytes(body.as_bytes())),
        };

        let rendered = format!("{error:?}");
        assert!(
            !rendered.contains("ROASTERY"),
            "response body leaked into Debug: {rendered}"
        );
        assert!(
            rendered.contains("redacted") && rendered.contains(&format!("{} bytes", body.len())),
            "Debug should say what it withheld and how big it was: {rendered}"
        );
    }

    /// The body is still there for a caller that deliberately asks — the point is that
    /// reaching it is a decision, not a side effect of logging.
    #[test]
    fn response_body_is_readable_on_purpose() {
        let body = ResponseBody::from_bytes(b"{\"not\":\"json\"");
        assert_eq!(body.as_str(), "{\"not\":\"json\"");
        assert_eq!(body.len(), 13);
        assert!(!body.is_empty());
        assert_eq!(body.into_inner(), "{\"not\":\"json\"");
    }

    /// The ceiling error names the cap either way, and the declared size when there was one,
    /// because "which of the two checks tripped" is the first thing you want to know.
    #[test]
    fn response_too_large_display_reports_the_cap_and_declared_size() {
        let declared = AkahuError::ResponseTooLarge {
            max_bytes: 1024,
            declared_bytes: Some(2048),
        }
        .to_string();
        assert!(declared.contains("1024"), "{declared}");
        assert!(declared.contains("2048"), "{declared}");

        let undeclared = AkahuError::ResponseTooLarge {
            max_bytes: 1024,
            declared_bytes: None,
        }
        .to_string();
        assert!(undeclared.contains("1024"), "{undeclared}");
        assert!(
            !undeclared.contains("Content-Length"),
            "should not mention a header the response didn't send: {undeclared}"
        );
    }
}
