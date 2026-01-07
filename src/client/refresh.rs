//! Data Refresh Endpoints
//!
//! This module contains methods for refreshing account data.

use crate::http::{HttpService, ServiceExt};
use crate::UserToken;

use super::AkahuClient;
use http::Method;

impl<H> AkahuClient<H>
where
    H: HttpService,
    H::Future: Send,
    H::Error: std::error::Error + Send + Sync + 'static,
{
    /// Refresh all accounts connected to your application.
    ///
    /// This endpoint initiates an on-demand data refresh across all accounts. Account data
    /// such as balance and transactions are periodically refreshed by Akahu and enriched
    /// asynchronously, providing clean and consistent data across financial institutions.
    ///
    /// **Note:** Data enrichment occurs asynchronously after the refresh request.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful refresh initiation.
    ///
    /// [<https://developers.akahu.nz/reference/post_refresh>]
    pub async fn refresh_all_accounts(
        &self,
        user_token: &UserToken,
    ) -> crate::error::AkahuResult<()> {
        const URI: &str = "refresh";

        let headers = self.build_user_headers(user_token)?;
        let url = format!("{}/{}", self.base_url, URI);

        let req = http::Request::builder()
            .method(Method::POST)
            .uri(url)
            .body(vec![])?;

        let (mut parts, body) = req.into_parts();
        parts.headers = headers;
        let req = http::Request::from_parts(parts, body);

        let res = self
            .client
            .call_cloned(req)
            .await
            .map_err(|e| crate::error::AkahuError::Http(Box::new(e)))?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_error_response(res)
        }
    }

    /// Refresh a specific account or connection.
    ///
    /// This endpoint initiates a data refresh for either a specific financial institution
    /// connection or individual account.
    ///
    /// **ID Type Behavior:**
    /// - **Connection ID**: Triggers a refresh for all accounts held at that financial institution
    /// - **Account ID**: Refreshes that specific account plus any other accounts sharing the
    ///   same login credentials. For example, if the user has three ASB accounts from a single
    ///   set of login credentials and you request a refresh for one, the other two accounts
    ///   will also be refreshed.
    ///
    /// **Note:** Data enrichment occurs asynchronously after the refresh request.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `id` - Either a Connection ID or Account ID
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful refresh initiation.
    ///
    /// [<https://developers.akahu.nz/reference/post_refresh-id>]
    pub async fn refresh_account_or_connection<Id: AsRef<str>>(
        &self,
        user_token: &UserToken,
        id: Id,
    ) -> crate::error::AkahuResult<()> {
        let uri = format!("refresh/{}", id.as_ref());
        let headers = self.build_user_headers(user_token)?;
        let url = format!("{}/{}", self.base_url, uri);

        let req = http::Request::builder()
            .method(Method::POST)
            .uri(url)
            .body(vec![])?;

        let (mut parts, body) = req.into_parts();
        parts.headers = headers;
        let req = http::Request::from_parts(parts, body);

        let res = self
            .client
            .call_cloned(req)
            .await
            .map_err(|e| crate::error::AkahuError::Http(Box::new(e)))?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_error_response(res)
        }
    }
}
