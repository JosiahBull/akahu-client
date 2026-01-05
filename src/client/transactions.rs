//! Transaction Endpoints
//!
//! This module contains methods for retrieving settled and pending transactions.

use crate::{AccountId, Transaction, TransactionCursor, TransactionQueryParams, PendingTransaction, UserToken};

use super::AkahuClient;
use reqwest::Method;
use std::collections::HashMap;

// TODO: Paginated responses should potentially be generic wrappers
#[derive(serde::Deserialize)]
pub struct PaginatedTransactionResponse {
    pub success: bool,
    pub items: Vec<Transaction>,
    pub cursor: TransactionCursor,
}

#[derive(serde::Deserialize)]
pub struct PaginatedPendingTransactionResponse {
    pub success: bool,
    pub items: Vec<PendingTransaction>,
    pub cursor: TransactionCursor,
}

impl AkahuClient {
    // ==================== Transaction Endpoints ====================

    /// Get a list of the user's settled transactions within a specified time range.
    ///
    /// This endpoint returns settled transactions for all accounts that the user has connected
    /// to your application. The response is paginated - use the `cursor.next` value to fetch
    /// subsequent pages.
    ///
    /// **Important Notes:**
    /// - Time range defaults to the entire range accessible to your app if not specified
    /// - Transactions will look different depending on your app's permissions
    /// - All transaction timestamps are in UTC
    /// - The start query parameter is exclusive (transactions after this timestamp)
    /// - The end query parameter is inclusive (transactions through this timestamp)
    /// - All Akahu timestamps use millisecond resolution (e.g. 2025-01-01T11:59:59.999Z)
    /// - Each page contains a maximum of 100 transactions
    /// - When querying multiple pages, use the same start/end parameters with the cursor
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `query` - Optional query parameters to filter by date range and paginate
    ///
    /// # Returns
    ///
    /// A paginated response containing transactions and a cursor for fetching more pages.
    ///
    /// [<https://developers.akahu.nz/reference/get_transactions>]
    pub async fn get_transactions(
        &self,
        user_token: &UserToken,
        query: Option<TransactionQueryParams>,
    ) -> crate::error::AkahuResult<PaginatedTransactionResponse> {
        const URI: &str = "transactions";

        let headers = self.build_user_headers(user_token)?;

        let url = if let Some(params) = query {
            let mut query_params = HashMap::new();

            if let Some(start) = params.start {
                query_params.insert(
                    "start",
                    start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                );
            }

            if let Some(end) = params.end {
                query_params.insert(
                    "end",
                    end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                );
            }

            if let Some(cursor) = params.cursor {
                query_params.insert("cursor", cursor.to_string());
            }

            let url = format!("{}/{}", self.base_url, URI);
            reqwest::Url::parse_with_params(&url, &query_params)?
        } else {
            reqwest::Url::parse(&format!("{}/{}", self.base_url, URI))?
        };

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get a list of the user's pending transactions.
    ///
    /// This endpoint returns pending transactions for all accounts that the user has connected
    /// to your application. Pending transactions are not stable - the date or description may
    /// change due to the unreliable nature of underlying NZ bank data. They are not assigned
    /// unique identifiers and are not enriched by Akahu.
    ///
    /// **Important Notes:**
    /// - Pending transactions may change before they settle
    /// - They do not have unique IDs
    /// - They are not enriched with merchant/category data
    /// - All timestamps are in UTC
    /// - The `updated_at` field indicates when the transaction was last fetched
    /// - Each page contains a maximum of 100 transactions
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `cursor` - Optional cursor for pagination (from previous response's `cursor.next`)
    ///
    /// # Returns
    ///
    /// A paginated response containing pending transactions and a cursor for fetching more pages.
    ///
    /// [<https://developers.akahu.nz/reference/get_transactions-pending>]
    pub async fn get_pending_transactions(
        &self,
        user_token: &UserToken,
        cursor: Option<String>,
    ) -> crate::error::AkahuResult<PaginatedPendingTransactionResponse> {
        const URI: &str = "transactions/pending";

        let headers = self.build_user_headers(user_token)?;

        let url = if let Some(cursor_val) = cursor {
            let mut query_params = HashMap::new();
            query_params.insert("cursor", cursor_val);

            let url = format!("{}/{}", self.base_url, URI);
            reqwest::Url::parse_with_params(&url, &query_params)?
        } else {
            reqwest::Url::parse(&format!("{}/{}", self.base_url, URI))?
        };

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get settled transactions for a specific account within a specified time range.
    ///
    /// This endpoint returns settled transactions for a specific connected account.
    /// The response is paginated - use the `cursor.next` value to fetch subsequent pages.
    ///
    /// **Important Notes:**
    /// - Time range defaults to the entire range accessible to your app if not specified
    /// - All transaction timestamps are in UTC
    /// - The start query parameter is exclusive (transactions after this timestamp)
    /// - The end query parameter is inclusive (transactions through this timestamp)
    /// - All Akahu timestamps use millisecond resolution
    /// - Each page contains a maximum of 100 transactions
    /// - When querying multiple pages, use the same start/end parameters with the cursor
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `account_id` - The unique identifier for the account (prefixed with `acc_`)
    /// * `query` - Optional query parameters to filter by date range and paginate
    ///
    /// # Returns
    ///
    /// A paginated response containing transactions and a cursor for fetching more pages.
    ///
    /// [<https://developers.akahu.nz/reference/get_accounts-id-transactions>]
    pub async fn get_account_transactions(
        &self,
        user_token: &UserToken,
        account_id: &AccountId,
        query: Option<TransactionQueryParams>,
    ) -> crate::error::AkahuResult<PaginatedTransactionResponse> {
        let uri = format!("accounts/{}/transactions", account_id.as_str());

        let headers = self.build_user_headers(user_token)?;

        let url = if let Some(params) = query {
            let mut query_params = HashMap::new();

            if let Some(start) = params.start {
                query_params.insert(
                    "start",
                    start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                );
            }

            if let Some(end) = params.end {
                query_params.insert(
                    "end",
                    end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                );
            }

            if let Some(cursor) = params.cursor {
                query_params.insert("cursor", cursor.to_string());
            }

            let url = format!("{}/{}", self.base_url, uri);
            reqwest::Url::parse_with_params(&url, &query_params)?
        } else {
            reqwest::Url::parse(&format!("{}/{}", self.base_url, uri))?
        };

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get pending transactions for a specific account.
    ///
    /// This endpoint returns pending transactions for a specific connected account.
    /// Pending transactions are not stable - the date or description may change due to
    /// the unreliable nature of underlying NZ bank data. They are not assigned unique
    /// identifiers and are not enriched by Akahu.
    ///
    /// **Important Notes:**
    /// - Pending transactions may change before they settle
    /// - They do not have unique IDs
    /// - They are not enriched with merchant/category data
    /// - All timestamps are in UTC
    /// - The `updated_at` field indicates when the transaction was last fetched
    /// - Each page contains a maximum of 100 transactions
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `account_id` - The unique identifier for the account (prefixed with `acc_`)
    /// * `cursor` - Optional cursor for pagination (from previous response's `cursor.next`)
    ///
    /// # Returns
    ///
    /// A paginated response containing pending transactions and a cursor for fetching more pages.
    ///
    /// [<https://developers.akahu.nz/reference/get_accounts-id-transactions-pending>]
    pub async fn get_account_pending_transactions(
        &self,
        user_token: &UserToken,
        account_id: &AccountId,
        cursor: Option<String>,
    ) -> crate::error::AkahuResult<PaginatedPendingTransactionResponse> {
        let uri = format!("accounts/{}/transactions/pending", account_id.as_str());

        let headers = self.build_user_headers(user_token)?;

        let url = if let Some(cursor_val) = cursor {
            let mut query_params = HashMap::new();
            query_params.insert("cursor", cursor_val);

            let url = format!("{}/{}", self.base_url, uri);
            reqwest::Url::parse_with_params(&url, &query_params)?
        } else {
            reqwest::Url::parse(&format!("{}/{}", self.base_url, uri))?
        };

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }
}
