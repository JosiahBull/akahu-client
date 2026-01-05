//! Authentication-related models for the Akahu API.

/// Request body for exchanging an authorization code for an access token.
///
/// This request must be made within 60 seconds of receiving the Authorization Code.
///
/// [<https://developers.akahu.nz/reference/post_token>]
#[derive(Debug, serde::Serialize, Clone)]
pub struct TokenExchangeRequest {
    /// Must be set to "authorization_code"
    pub grant_type: String,

    /// The authorization code received from the OAuth flow
    pub code: String,

    /// The redirect URI used in the authorization request
    pub redirect_uri: String,

    /// Your application's App ID Token
    pub client_id: String,

    /// Your application's App Secret
    pub client_secret: String,
}

/// Response from a successful token exchange.
///
/// [<https://developers.akahu.nz/reference/post_token>]
#[derive(Debug, serde::Deserialize, Clone)]
pub struct TokenExchangeResponse {
    /// Indicates if the request was successful
    pub success: bool,

    /// The user access token that can be used to access the API
    pub access_token: String,

    /// The type of token, typically "bearer"
    pub token_type: String,

    /// Space-separated list of scopes granted
    pub scope: String,
}

/// Error response from the token endpoint.
///
/// Note: Error responses from the token endpoint use the `error` field
/// rather than the `message` field used by other Akahu endpoints.
///
/// [<https://developers.akahu.nz/reference/post_token>]
#[derive(Debug, serde::Deserialize, Clone)]
pub struct TokenErrorResponse {
    /// Indicates the request failed
    pub success: bool,

    /// Error code following OAuth2 specification
    pub error: String,

    /// Optional human-readable error description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}
