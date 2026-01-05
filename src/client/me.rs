//! User profile endpoint implementations.
//!
//! This module contains methods for retrieving authenticated user information.

use super::AkahuClient;
use reqwest::Method;

impl AkahuClient {
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
    /// - User ID (always prefixed with `user_`)
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
    /// # Example
    ///
    /// ```no_run
    /// # use akahu_client::AkahuClient;
    /// # async fn example(client: AkahuClient, user_token: &str) {
    /// let user = client.get_me(user_token).await;
    /// println!("User ID: {}", user.id);
    /// if let Some(email) = user.email {
    ///     println!("Email: {}", email);
    /// }
    /// # }
    /// ```
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    pub async fn get_me(&self, user_token: &str) -> crate::error::AkahuResult<crate::models::User> {
        const URI: &str = "me";

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        let user_response: crate::models::UserResponse = self.execute_request(req).await?;

        Ok(user_response.item)
    }
}
