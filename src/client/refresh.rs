//! Data Refresh Endpoints
//!
//! This module contains methods for refreshing account data.

use crate::{AccountId, UserToken};

use super::AkahuClient;
use reqwest::Method;

impl AkahuClient {
    // ==================== Data Refresh Endpoints ====================

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

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_error_response(res).await
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
    pub async fn refresh_account_or_connection(
        &self,
        user_token: &UserToken,
        id: &str,
    ) -> crate::error::AkahuResult<()> {
        let uri = format!("refresh/{}", id);

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, uri))
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
