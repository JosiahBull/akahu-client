//! Account endpoint implementations.
//!
//! This module contains methods for managing user accounts connected to your Akahu application.

use super::AkahuClient;
use reqwest::Method;

impl AkahuClient {
    /// Get a list of all accounts that the user has connected to your application.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    ///
    /// # Returns
    ///
    /// A vector of all accounts the user has connected to your application.
    /// The returned account data depends on the permissions your application has been granted.
    ///
    /// [<https://developers.akahu.nz/reference/get_accounts>]
    pub async fn get_accounts(
        &self,
        user_token: &str,
    ) -> crate::error::AkahuResult<Vec<crate::models::Account>> {
        const URI: &str = "accounts";

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get a specific account by its ID.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `account_id` - The unique identifier for the account (prefixed with `acc_`)
    ///
    /// # Returns
    ///
    /// The account details for the specified account.
    /// The returned account data depends on the permissions your application has been granted.
    ///
    /// [<https://developers.akahu.nz/reference/get_accounts-id>]
    pub async fn get_account(
        &self,
        user_token: &str,
        account_id: &str,
    ) -> crate::error::AkahuResult<crate::models::Account> {
        let uri = format!("accounts/{}", account_id);

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Revoke your application's access to a specific account.
    ///
    /// **Note:** This endpoint is deprecated for accounts with official open banking connections.
    /// Accounts connected via official open banking cannot be revoked on an individual basis.
    /// Instead, you must either:
    /// - Direct users through the OAuth flow to adjust permissions with their bank
    /// - Use the Revoke Access To Authorisation endpoint to revoke the entire authorization
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `account_id` - The unique identifier for the account to revoke access from
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful revocation.
    /// Returns an error (400) when attempting to revoke accounts with `connection_type` of "official".
    ///
    /// [<https://developers.akahu.nz/reference/delete_accounts-id>]
    pub async fn revoke_account_access(
        &self,
        user_token: &str,
        account_id: &str,
    ) -> crate::error::AkahuResult<()> {
        let uri = format!("accounts/{}", account_id);

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::DELETE, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        // This endpoint returns empty response on success
        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_error_response(res).await
        }
    }
}
