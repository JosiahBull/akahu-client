//! Reqwest HTTP client implementation
//!
//! This module provides an implementation of `tower::Service` using the `reqwest` library.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

/// HTTP client implementation using reqwest that implements tower::Service.
///
/// This is the default HTTP client implementation when the `reqwest` feature is enabled.
/// The client is cheap to clone (uses `Arc` internally) and implements `tower::Service`.
///
/// # Example
///
/// ```rust,ignore
/// use akahu_client::{AkahuClient, ReqwestClient};
/// use tower::ServiceExt;
///
/// let http_client = ReqwestClient::new(reqwest::Client::new());
/// let client = AkahuClient::new(http_client, app_token, None);
/// ```
#[derive(Debug, Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    /// Create a new reqwest client wrapper
    ///
    /// # Arguments
    ///
    /// * `client` - The underlying reqwest::Client to use
    #[allow(clippy::missing_const_for_fn, reason = "reqwest::Client is not const")]
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl Service<http::Request<Vec<u8>>> for ReqwestClient {
    type Response = http::Response<Vec<u8>>;
    type Error = reqwest::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // HTTP clients are always ready to accept requests
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: http::Request<Vec<u8>>) -> Self::Future {
        let client = self.client.clone();

        Box::pin(async move {
            // Convert http::Request to reqwest::Request
            let (parts, body) = request.into_parts();

            // Convert http::Method to reqwest::Method
            // This should always succeed for valid HTTP methods
            let method = reqwest::Method::from_bytes(parts.method.as_str().as_bytes())
                .expect("Valid HTTP method");

            let url = parts.uri.to_string();

            let mut req_builder = client.request(method, &url);

            // Add headers
            for (name, value) in parts.headers.iter() {
                req_builder = req_builder.header(name.as_str(), value.as_bytes());
            }

            // Add body if not empty
            if !body.is_empty() {
                req_builder = req_builder.body(body);
            }

            let req = req_builder.build()?;

            // Execute the request
            let response = client.execute(req).await?;

            // Convert reqwest::Response to http::Response
            let status = response.status();
            let headers = response.headers().clone();
            let body_bytes = response.bytes().await?;

            let mut http_response = http::Response::builder().status(status.as_u16());

            // Add headers to response
            for (name, value) in headers.iter() {
                http_response = http_response.header(name.as_str(), value.as_bytes());
            }

            let http_response = http_response
                .body(body_bytes.to_vec())
                .expect("Failed to build HTTP response");

            Ok(http_response)
        })
    }
}
