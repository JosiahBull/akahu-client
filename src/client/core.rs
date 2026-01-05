//! Core helper methods for the Akahu client.
//!
//! This module contains internal methods for request execution, error handling,
//! and header construction. These methods are used by all endpoint implementations.

use super::AkahuClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};

const AKAHU_ID_HEADER: &str = "X-Akahu-Id";

impl AkahuClient {
    /// Execute a request and handle the response, converting HTTP errors to AkahuError
    pub(super) async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::Request,
    ) -> crate::error::AkahuResult<T> {
        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            self.handle_error_response(res).await
        }
    }

    /// Parse error response and map to appropriate AkahuError variant
    pub(super) async fn handle_error_response<T>(
        &self,
        res: reqwest::Response,
    ) -> crate::error::AkahuResult<T> {
        let status = res.status();
        let status_code = status.as_u16();

        // Try to parse error message from response body
        let message = match res.json::<crate::models::ErrorResponse>().await {
            Ok(error_body) => error_body.message,
            Err(_) => status
                .canonical_reason()
                .unwrap_or("Unknown error")
                .to_string(),
        };

        Err(match status_code {
            400 => crate::error::AkahuError::BadRequest {
                message,
                status: status_code,
            },
            401 => crate::error::AkahuError::Unauthorized { message },
            403 => crate::error::AkahuError::Forbidden { message },
            404 => crate::error::AkahuError::NotFound { message },
            429 => crate::error::AkahuError::RateLimited { message },
            500 => crate::error::AkahuError::InternalServerError { message },
            _ => crate::error::AkahuError::ApiError {
                status: status_code,
                message,
            },
        })
    }

    /// Build standard headers for user-scoped requests
    pub(super) fn build_user_headers(
        &self,
        user_token: &str,
    ) -> crate::error::AkahuResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(AKAHU_ID_HEADER, HeaderValue::from_str(&self.app_id_token)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", user_token))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    /// Build headers for app-scoped requests (HTTP Basic Auth)
    pub(super) fn build_app_headers(&self) -> crate::error::AkahuResult<HeaderMap> {
        let app_secret = self
            .app_secret
            .as_ref()
            .ok_or(crate::error::AkahuError::MissingAppSecret)?;

        let credentials = format!("{}:{}", self.app_id_token, app_secret);
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            credentials.as_bytes(),
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}
