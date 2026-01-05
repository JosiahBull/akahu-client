//! Payments Endpoints
//!
//! This module contains methods for initiating and managing payments to New Zealand bank accounts.

use super::AkahuClient;
use reqwest::{Method, header::HeaderValue};
use std::collections::HashMap;

impl AkahuClient {
    // ==================== Payments Endpoints ====================

    /// Get a list of payments initiated by your application within a specified timeframe.
    ///
    /// This endpoint retrieves all payments that your application has created on behalf
    /// of the user. The default time range is the last 30 days if no parameters are provided.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `start` - Optional start of the time range (exclusive) in ISO 8601 format
    /// * `end` - Optional end of the time range (inclusive) in ISO 8601 format
    ///
    /// # Returns
    ///
    /// A vector of all payments created by your application for this user.
    ///
    /// # Notes
    ///
    /// - Time range defaults to the last 30 days
    /// - All timestamps are in UTC
    /// - The start parameter is exclusive
    /// - The end parameter is inclusive
    ///
    /// [<https://developers.akahu.nz/reference/get_payments>]
    pub async fn get_payments(
        &self,
        user_token: &str,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> crate::error::AkahuResult<Vec<crate::models::Payment>> {
        const URI: &str = "payments";

        let headers = self.build_user_headers(user_token)?;

        let mut query_params = HashMap::new();
        if let Some(start_time) = start {
            query_params.insert(
                "start",
                start_time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            );
        }
        if let Some(end_time) = end {
            query_params.insert(
                "end",
                end_time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            );
        }

        let url = format!("{}/{}", self.base_url, URI);
        let url = if query_params.is_empty() {
            reqwest::Url::parse(&url)?
        } else {
            reqwest::Url::parse_with_params(&url, &query_params)?
        };

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()?;

        let response: crate::models::PaymentsListResponse = self.execute_request(req).await?;

        Ok(response.items)
    }

    /// Initiate a payment from the user's connected bank account to another New Zealand bank account.
    ///
    /// This endpoint creates a payment from a user's account to any New Zealand bank account.
    /// The receiving account does not need to be connected to Akahu.
    ///
    /// **Important Notes:**
    /// - Your application must be whitelisted for payment functionality
    /// - The source account must have the `PAYMENT_FROM` attribute
    /// - Maximum single payment: $100,000 (Akahu limit)
    /// - Bank-specific daily and single transaction limits apply
    /// - Receiving a success response does not guarantee payment completion
    /// - Monitor payment status via webhooks or by polling the GET /payments/:id endpoint
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `request` - The payment request containing source account, destination details, amount, and optional metadata
    ///
    /// # Returns
    ///
    /// Returns the created payment object with its initial status.
    ///
    /// # Payment Lifecycle
    ///
    /// Payments go through several states:
    /// - **READY**: Payment is ready to be processed
    /// - **PENDING_APPROVAL**: Awaiting approval (via bank or MyAkahu)
    /// - **PAUSED**: Processing is paused
    /// - **SENT**: Successfully sent (final)
    /// - **DECLINED**: Declined by bank or Akahu (final)
    /// - **ERROR**: Processing error (final)
    /// - **CANCELLED**: Cancelled by user, app, or system (final)
    ///
    /// [<https://developers.akahu.nz/reference/post_payments>]
    pub async fn create_payment(
        &self,
        user_token: &str,
        request: crate::models::CreatePaymentRequest,
    ) -> crate::error::AkahuResult<crate::models::Payment> {
        const URI: &str = "payments";

        let mut headers = self.build_user_headers(user_token)?;
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .json(&request)
            .build()?;

        let response: crate::models::PaymentResponse = self.execute_request(req).await?;

        Ok(response.item)
    }

    /// Initiate a tax payment from the user's connected bank account to the Inland Revenue Department (IRD).
    ///
    /// This is a specialized endpoint for making tax payments to New Zealand's IRD.
    /// Use this endpoint instead of the standard `/payments` endpoint for IRD payments.
    ///
    /// **Important Notes:**
    /// - Your application must be whitelisted for payment functionality
    /// - The source account must have the `PAYMENT_FROM` attribute
    /// - Payment limits and restrictions apply as with standard payments
    /// - Monitor payment status via webhooks or by polling the GET /payments/:id endpoint
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `request` - The IRD payment request containing source account, amount, and optional metadata
    ///
    /// # Returns
    ///
    /// Returns the created payment object with its initial status.
    ///
    /// [<https://developers.akahu.nz/reference/post_payments-ird>]
    pub async fn create_ird_payment(
        &self,
        user_token: &str,
        request: crate::models::CreateIrdPaymentRequest,
    ) -> crate::error::AkahuResult<crate::models::Payment> {
        const URI: &str = "payments/ird";

        let mut headers = self.build_user_headers(user_token)?;
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::POST, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .json(&request)
            .build()?;

        let response: crate::models::PaymentResponse = self.execute_request(req).await?;

        Ok(response.item)
    }

    /// Get details about a specific payment by its ID.
    ///
    /// This endpoint retrieves the current state and details of an individual payment
    /// that your application has created.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `payment_id` - The unique identifier for the payment (prefixed with payment ID format)
    ///
    /// # Returns
    ///
    /// The payment details including current status, timeline, and metadata.
    ///
    /// # Use Cases
    ///
    /// - Poll this endpoint to check payment status if not using webhooks
    /// - Retrieve payment details to display to users
    /// - Check for approval URLs when status is PENDING_APPROVAL
    /// - Verify final payment status and any error messages
    ///
    /// [<https://developers.akahu.nz/reference/get_payments-id>]
    pub async fn get_payment(
        &self,
        user_token: &str,
        payment_id: &str,
    ) -> crate::error::AkahuResult<crate::models::Payment> {
        let uri = format!("payments/{}", payment_id);

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        let response: crate::models::PaymentResponse = self.execute_request(req).await?;

        Ok(response.item)
    }

    /// Cancel a payment that is currently in the PENDING_APPROVAL state.
    ///
    /// This endpoint allows you to cancel a payment that is awaiting user approval.
    /// Payments can only be cancelled when they are in the `PENDING_APPROVAL` status.
    ///
    /// # Arguments
    ///
    /// * `user_token` - The user's access token obtained through OAuth
    /// * `payment_id` - The unique identifier for the payment to cancel
    ///
    /// # Returns
    ///
    /// Returns the updated payment object with status changed to `CANCELLED`.
    ///
    /// # Errors
    ///
    /// This endpoint will fail if:
    /// - The payment is not in `PENDING_APPROVAL` state
    /// - The payment has already been processed or cancelled
    /// - The payment ID is invalid or belongs to a different user
    ///
    /// # Notes
    ///
    /// - Only payments in `PENDING_APPROVAL` state can be cancelled
    /// - Once cancelled, the payment enters a final state and cannot be reactivated
    /// - The cancellation is immediate and cannot be undone
    ///
    /// [<https://developers.akahu.nz/reference/put_payments-id-cancel>]
    pub async fn cancel_payment(
        &self,
        user_token: &str,
        payment_id: &str,
    ) -> crate::error::AkahuResult<crate::models::Payment> {
        let uri = format!("payments/{}/cancel", payment_id);

        let headers = self.build_user_headers(user_token)?;

        let req = self
            .client
            .request(Method::PUT, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        let response: crate::models::PaymentResponse = self.execute_request(req).await?;

        Ok(response.item)
    }
}
