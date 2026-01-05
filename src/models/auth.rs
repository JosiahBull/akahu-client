//! Authentication-related models for the Akahu API.

use serde::{Deserialize, Serialize};

use crate::{AppToken, AuthCode, ClientSecret, RedirectUri, UserToken};

/// Request body for exchanging an authorization code for an access token.
///
/// This request must be made within 60 seconds of receiving the Authorization Code.
///
/// [<https://developers.akahu.nz/reference/post_token>]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TokenExchangeRequest {
    /// Must be set to "authorization_code"
    pub grant_type: String,

    /// The authorization code received from the OAuth flow
    pub code: AuthCode,

    /// The redirect URI used in the authorization request
    pub redirect_uri: RedirectUri,

    /// Your application's App ID Token
    pub client_id: AppToken,

    /// Your application's App Secret
    pub client_secret: ClientSecret,
}

/// Response from a successful token exchange.
///
/// [<https://developers.akahu.nz/reference/post_token>]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TokenExchangeResponse {
    /// Indicates if the request was successful
    pub success: bool,

    /// The user access token that can be used to access the API
    pub access_token: UserToken,

    /// The type of token, typically "bearer"
    pub token_type: String,

    /// Space-separated list of scopes granted
    pub scope: String,
}
