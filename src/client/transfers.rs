//! Transfers Endpoints
//!
//! This module contains methods for managing transfers between user accounts.

use crate::{TransferId, UserToken};

use super::AkahuClient;
use reqwest::{Method, header::HeaderValue};
use std::collections::HashMap;

// TODO: These should be replaced with function parameters using bon builders
#[derive(serde::Serialize)]
struct TransferQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(serde::Serialize)]
struct TransferCreateParams {
    // Placeholder - fields need to be added based on API requirements
}

impl AkahuClient {
    // ==================== Transfers Endpoints ====================

    /// Get a list of transfers that your application has initiated between the user's connected accounts.
    ///
    /// This endpoint returns all transfers within the specified date range. If no range is provided,
    /// it defaults to the last 30 days.
    ///
    /// **Important Notes:**
    /// - Only returns transfers initiated by your application
    /// - Transfers are between accounts belonging to the same user
    /// - All timestamps are in UTC
    /// - The start query parameter is exclusive
    /// - The end query parameter is inclusive
    /// - All Akahu timestamps use millisecond resolution
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `query` - Optional query parameters to filter transfers by date range
    ///
    /// # Returns
    ///
    /// A vector of all transfers matching the query parameters.
    ///
    /// [<https://developers.akahu.nz/reference/get_transfers>]
    pub async fn get_transfers(
        &self,
        user_token: &UserToken,
        query: Option<TransferQueryParams>,
    ) -> crate::error::AkahuResult<Vec<crate::models::Transfer>> {
        const URI: &str = "transfers";

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

    /// Create a new transfer between two of the user's connected accounts.
    ///
    /// This endpoint initiates a transfer of funds from one account to another account
    /// belonging to the same user. The transfer will be processed asynchronously, and you
    /// can track its progress using the `status` and `timeline` fields.
    ///
    /// **Important Notes:**
    /// - Both accounts must belong to the same user
    /// - The source account must have the `TRANSFER_FROM` attribute
    /// - The destination account must have the `TRANSFER_TO` attribute
    /// - The transfer begins with status `READY` and progresses through various states
    /// - Use webhooks or polling to track transfer completion
    /// - The `final` field indicates when the transfer will no longer update
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `params` - The transfer parameters (from, to, amount)
    ///
    /// # Returns
    ///
    /// The created transfer object with its initial status and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Either account doesn't exist or doesn't belong to the user
    /// - The accounts don't have the required transfer attributes
    /// - The amount is invalid or exceeds available balance
    /// - The accounts are from incompatible institutions
    ///
    /// [<https://developers.akahu.nz/reference/post_transfers>]
    pub async fn create_transfer(
        &self,
        user_token: &UserToken,
        params: TransferCreateParams,
    ) -> crate::error::AkahuResult<crate::models::Transfer> {
        const URI: &str = "transfers";

        let mut headers = self.build_user_headers(user_token)?;
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .json(&params)
            .build()?;

        self.execute_request(req).await
    }

    /// Get details for a specific transfer by its ID.
    ///
    /// This endpoint retrieves the current state and full details of a transfer that
    /// your application has initiated.
    ///
    /// **Use Cases:**
    /// - Checking the current status of a transfer
    /// - Reviewing the complete timeline of status changes
    /// - Verifying transfer completion (when `final` is true)
    /// - Retrieving error details (via `status_text`)
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `transfer_id` - The unique identifier for the transfer (prefixed with `transfer_`)
    ///
    /// # Returns
    ///
    /// The transfer object with its current status and complete metadata.
    ///
    /// [<https://developers.akahu.nz/reference/get_transfers-id>]
    pub async fn get_transfer(
        &self,
        user_token: &UserToken,
        transfer_id: &TransferId,
    ) -> crate::error::AkahuResult<crate::models::Transfer> {
        let uri = format!("transfers/{}", transfer_id.as_str());

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }
}
