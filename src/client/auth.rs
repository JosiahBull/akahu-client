//! Authentication endpoint implementations.
//!
//! This module contains methods for OAuth2 authentication flows.

use crate::{AuthCode, ClientSecret, ErrorResponse, RedirectUri, UserToken};

use super::AkahuClient;
use reqwest::Method;
use reqwest::header::{HeaderMap, HeaderValue};

impl AkahuClient {
    /// Exchange an authorization code for a user access token.
    ///
    /// This endpoint completes the OAuth2 authentication flow by exchanging a temporary
    /// authorization code for a persistent user access token.
    ///
    /// **Important:** This request must be made within 60 seconds of receiving the authorization code.
    ///
    /// **Note:** This endpoint is not applicable for Personal Apps. Use the Getting Started guide
    /// for Personal App setup instead.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code received from the OAuth flow
    /// * `redirect_uri` - The redirect URI used in the authorization request
    /// * `client_secret` - Your application's App Secret
    ///
    /// # Returns
    ///
    /// Returns a `TokenExchangeResponse` containing the access token and granted scopes.
    ///
    /// # Errors
    ///
    /// Error responses from this endpoint follow OAuth2 specifications and contain an `error` field
    /// rather than the `message` field used by other Akahu endpoints.
    ///
    /// [<https://developers.akahu.nz/reference/post_token>]
    pub async fn exchange_authorization_code(
        &self,
        code: &AuthCode,
        redirect_uri: &RedirectUri,
        client_secret: &ClientSecret,
    ) -> crate::error::AkahuResult<crate::models::TokenExchangeResponse> {
        const URI: &str = "token";

        let request_body = crate::models::TokenExchangeRequest {
            grant_type: "authorization_code".to_string(),
            code: code.clone(),
            redirect_uri: redirect_uri.clone(),
            client_id: self.app_id_token.clone(),
            client_secret: client_secret.clone(),
        };

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .json(&request_body)
            .build()?;

        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            let status = res.status().as_u16();
            let error_response: ErrorResponse = res.json().await?;
            Err(crate::error::AkahuError::ApiError {
                status,
                message: error_response.message,
            })
        }
    }

    /// Revoke a user access token.
    ///
    /// This endpoint revokes the user access token provided in the request, removing your
    /// application's access to all of the user's connected account data including transactions.
    ///
    /// Users can subsequently re-authorize your application through the OAuth2 authorization flow.
    ///
    /// **Important:** Tokens should be revoked when they are no longer needed, such as when a
    /// user deletes their account in your application.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user access token to revoke
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful revocation.
    ///
    /// [<https://developers.akahu.nz/reference/delete_token>]
    pub async fn revoke_token(&self, user_token: &UserToken) -> crate::error::AkahuResult<()> {
        const URI: &str = "token";

        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", user_token.as_str()))?,
        );
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::DELETE, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_error_response(res).await
        }
    }
}
