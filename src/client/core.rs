//! Core helper methods for the Akahu client.

use crate::http::{HttpService, ServiceExt};
use crate::UserToken;

use super::AkahuClient;
use http::{
    StatusCode,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};

impl<H> AkahuClient<H>
where
    H: HttpService,
    H::Error: std::error::Error + Send + Sync + 'static,
    H::Future: Send,
{
    /// Execute a request and handle the response, converting HTTP errors to AkahuError
    pub(super) async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        req: http::Request<Vec<u8>>,
    ) -> crate::error::AkahuResult<T> {
        let res = self
            .client
            .call_cloned(req)
            .await
            .map_err(|e| crate::error::AkahuError::Http(Box::new(e)))?;

        if res.status().is_success() {
            let body = res.into_body();
            let text = String::from_utf8_lossy(&body).to_string();
            // Try to deserialize into the expected type T
            let deserialized: T = serde_json::from_str(&text).map_err(|e| {
                crate::error::AkahuError::JsonDeserialization {
                    error: e,
                    source_string: Some(text),
                }
            })?;
            Ok(deserialized)
        } else {
            self.handle_error_response(res)
        }
    }

    /// Parse error response and map to appropriate AkahuError variant
    #[allow(clippy::unused_self, reason = "Keep consistent method signature")]
    pub(super) fn handle_error_response<T>(
        &self,
        res: http::Response<Vec<u8>>,
    ) -> crate::error::AkahuResult<T> {
        let status = res.status();
        let body = res.into_body();

        // Try to parse error message from response body
        let message = match serde_json::from_slice::<crate::models::ErrorResponse>(&body) {
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

        let akahu_id_name = http::header::HeaderName::from_static("x-akahu-id");
        let akahu_id_value = HeaderValue::from_str(&self.app_id_token)
            .map_err(|e| crate::error::AkahuError::InvalidHeader(e.to_string()))?;
        headers.insert(akahu_id_name, akahu_id_value);

        let auth_value = HeaderValue::from_str(&format!("Bearer {}", user_token.as_str()))
            .map_err(|e| crate::error::AkahuError::InvalidHeader(e.to_string()))?;
        headers.insert(AUTHORIZATION, auth_value);

        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}
