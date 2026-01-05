//! Support Endpoints
//!
//! This module contains methods for reporting issues with transaction data.

use super::AkahuClient;
use reqwest::{Method, header::HeaderValue};

impl AkahuClient {
    // ==================== Support Endpoints ====================

    /// Report an issue with a transaction.
    ///
    /// This endpoint allows you to report issues with Akahu transactions, including
    /// duplicates, enrichment errors, and enrichment suggestions. This helps improve
    /// the quality and accuracy of transaction data.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `transaction_id` - The unique identifier for the transaction (prefixed with `trans_`)
    /// * `request` - The support request specifying the type of issue being reported
    ///
    /// # Returns
    ///
    /// Returns `Ok(TransactionSupportResponse)` on successful submission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use akahu_client::{AkahuClient, TransactionSupportRequest};
    /// # async fn example(client: AkahuClient, user_token: &str) {
    /// // Report a duplicate transaction
    /// let request = TransactionSupportRequest::Duplicate {
    ///     other_id: "trans_456def...".to_string(),
    /// };
    /// client.report_transaction_issue(user_token, "trans_123abc...", request).await;
    ///
    /// // Report an enrichment error
    /// let request = TransactionSupportRequest::EnrichmentError {
    ///     fields: vec!["merchant.name".to_string()],
    ///     comment: "Merchant name is incorrect".to_string(),
    /// };
    /// client.report_transaction_issue(user_token, "trans_123abc...", request).await;
    ///
    /// // Suggest an enrichment improvement
    /// let request = TransactionSupportRequest::EnrichmentSuggestion {
    ///     comment: "Could add category for this merchant".to_string(),
    /// };
    /// client.report_transaction_issue(user_token, "trans_123abc...", request).await;
    /// # }
    /// ```
    ///
    /// [<https://developers.akahu.nz/reference/post_support-transaction-id>]
    pub async fn report_transaction_issue(
        &self,
        user_token: &str,
        transaction_id: &str,
        request: crate::models::TransactionSupportRequest,
    ) -> crate::error::AkahuResult<crate::models::TransactionSupportResponse> {
        let uri = format!("support/transaction/{}", transaction_id);

        let mut headers = self.build_user_headers(user_token)?;
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .json(&request)
            .build()?;

        self.execute_request(req).await
    }
}
