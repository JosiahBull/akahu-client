//! Core helper methods for the Akahu client.

use crate::UserToken;

use super::AkahuClient;
use reqwest::{
    StatusCode,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};

const AKAHU_ID_HEADER: &str = "X-Akahu-Id";

impl AkahuClient {
    /// Execute a request and handle the response, converting HTTP errors to AkahuError
    pub(super) async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::Request,
    ) -> crate::error::AkahuResult<T> {
        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            let text = res.text().await?;
            // Try to deserialize into the expected type T
            let deserialized: T = serde_json::from_str(&text).map_err(|e| {
                crate::error::AkahuError::JsonDeserialization {
                    error: e,
                    source_string: Some(text),
                }
            })?;
            Ok(deserialized)
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

        // Try to parse error message from response body
        let message = match res.json::<crate::models::ErrorResponse>().await {
            Ok(error_body) => error_body.message,
            Err(_) => status
                .canonical_reason()
                .unwrap_or("Unknown error")
                .to_string(),
        };

        Err(match status {
            StatusCode::BAD_REQUEST => crate::error::AkahuError::BadRequest {
                message,
                status: StatusCode::BAD_REQUEST.as_u16(),
            },
            StatusCode::UNAUTHORIZED => crate::error::AkahuError::Unauthorized { message },
            StatusCode::FORBIDDEN => crate::error::AkahuError::Forbidden { message },
            StatusCode::NOT_FOUND => crate::error::AkahuError::NotFound { message },
            StatusCode::TOO_MANY_REQUESTS => crate::error::AkahuError::RateLimited { message },
            StatusCode::INTERNAL_SERVER_ERROR => {
                crate::error::AkahuError::InternalServerError { message }
            }
            _ => crate::error::AkahuError::ApiError {
                status: status.as_u16(),
                message,
            },
        })
    }

    /// Build standard headers for user-scoped requests
    pub(super) fn build_user_headers(
        &self,
        user_token: &UserToken,
    ) -> crate::error::AkahuResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(AKAHU_ID_HEADER, HeaderValue::from_str(&self.app_id_token)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", user_token.as_str()))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}
