//! Akahu API client implementation.

#![allow(
    clippy::multiple_inherent_impl,
    reason = "Organized into separate files by endpoint type for maintainability"
)]

mod accounts;
mod core;
mod me;
mod refresh;
mod transactions;

use crate::http::HttpService;
use crate::{AppSecret, AppToken};

#[cfg(feature = "reqwest")]
use crate::ReqwestClient;

/// Default base URL for the Akahu API
const DEFAULT_BASE_URL: &str = "https://api.akahu.io/v1";

/// Type alias for the default HTTP client when reqwest feature is enabled
#[cfg(feature = "reqwest")]
pub type DefaultHttpClient = ReqwestClient;

/// The main Akahu API client.
///
/// This client is generic over the HTTP client implementation, allowing you to use
/// any HTTP library that implements `tower::Service`.
///
/// When the `reqwest` feature is enabled (default), the type parameter defaults to
/// `ReqwestClient`, making it easy to use without specifying the HTTP client type.
///
/// When the `reqwest` feature is disabled, you must explicitly specify the HTTP client type.
#[cfg(feature = "reqwest")]
pub struct AkahuClient<H = DefaultHttpClient> {
    /// HTTP client for making requests
    client: H,
    /// Application ID token
    app_id_token: AppToken,
    /// Optional application secret for app-scoped endpoints
    app_secret: Option<AppSecret>,
    /// Base URL for API requests
    base_url: String,
}

/// The main Akahu API client (without default HTTP client).
///
/// This version is used when the `reqwest` feature is disabled.
/// You must explicitly specify the HTTP client type that implements `tower::Service`.
#[cfg(not(feature = "reqwest"))]
pub struct AkahuClient<H> {
    /// HTTP client for making requests
    client: H,
    /// Application ID token
    app_id_token: AppToken,
    /// Optional application secret for app-scoped endpoints
    app_secret: Option<AppSecret>,
    /// Base URL for API requests
    base_url: String,
}

impl<H> AkahuClient<H>
where
    H: HttpService,
    H::Error: std::error::Error + Send + Sync + 'static,
    H::Future: Send,
{
    /// Create a new Akahu client with a custom HTTP client implementation.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use for requests (must implement `HttpClient` trait)
    /// * `app_id_token` - Your Akahu application ID token
    /// * `base_url` - Optional custom base URL (defaults to `https://api.akahu.io/v1`)
    pub fn new<T: Into<AppToken>>(client: H, app_id_token: T, base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        Self {
            client,
            app_id_token: app_id_token.into(),
            app_secret: None,
            base_url,
        }
    }

    /// Set the app secret for app-scoped endpoints.
    ///
    /// The app secret is required for app-scoped endpoints like Categories.
    /// These endpoints use HTTP Basic Authentication with app_id_token:app_secret.
    pub fn with_app_secret<T: Into<AppSecret>>(mut self, app_secret: T) -> Self {
        self.app_secret = Some(app_secret.into());
        self
    }
}

#[cfg(feature = "reqwest")]
impl AkahuClient<ReqwestClient> {
    /// Create a new Akahu client using the default reqwest HTTP client.
    ///
    /// This is a convenience method that's only available when the `reqwest` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `app_id_token` - Your Akahu application ID token
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use akahu_client::AkahuClient;
    ///
    /// let client = AkahuClient::with_reqwest("app_token_...");
    /// ```
    pub fn with_reqwest<T: Into<AppToken>>(app_id_token: T) -> Self {
        Self::new(ReqwestClient::default(), app_id_token, None)
    }
}
