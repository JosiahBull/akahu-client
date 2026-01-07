//! User profile endpoint implementations.
//!
//! This module contains methods for retrieving authenticated user information.

use crate::http::HttpService;
use crate::{ItemResponse, User, UserToken};

use super::AkahuClient;
use http::Method;

impl<H> AkahuClient<H>
where
    H: HttpService,
    H::Future: Send,
    H::Error: std::error::Error + Send + Sync + 'static,
{
    /// Get the authenticated user's profile information.
    ///
    /// This endpoint retrieves information about the user who authorized your application,
    /// including their unique identifier and basic profile data. The visibility of certain
    /// fields depends on the permissions granted to your application.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    ///
    /// # Returns
    ///
    /// The user's profile information, including:
    /// - User ID
    /// - Account creation timestamp
    /// - First name (if available)
    /// - Last name (if available)
    /// - Email address (requires `AKAHU` scope)
    /// - Access granted timestamp (when the user authorized your app)
    ///
    /// # Scopes
    ///
    /// This endpoint requires user-scoped authentication. The `AKAHU` scope is needed
    /// to access the user's email address and other profile information.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    pub async fn get_me(
        &self,
        user_token: &UserToken,
    ) -> crate::error::AkahuResult<crate::models::User> {
        const URI: &str = "me";

        let headers = self.build_user_headers(user_token)?;
        let url = format!("{}/{}", self.base_url, URI);

        let req = http::Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(vec![])?;

        let (mut parts, body) = req.into_parts();
        parts.headers = headers;
        let req = http::Request::from_parts(parts, body);

        let user_response: ItemResponse<User> = self.execute_request(req).await?;

        Ok(user_response.item)
    }
}
