//! Akahu API client implementation.

mod accounts;
mod core;
mod me;
mod refresh;
mod transactions;

use crate::{AppSecret, AppToken};

/// Default base URL for the Akahu API
const DEFAULT_BASE_URL: &str = "https://api.akahu.io/v1";

/// The main Akahu API client.
///
/// Use the builder pattern to construct a new client.
pub struct AkahuClient {
    /// HTTP client for making requests
    client: reqwest::Client,
    /// Application ID token
    app_id_token: AppToken,
    /// Optional application secret for app-scoped endpoints
    app_secret: Option<AppSecret>,
    /// Base URL for API requests
    base_url: String,
}

impl AkahuClient {
    /// Create a new Akahu client.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use for requests
    /// * `app_id_token` - Your Akahu application ID token
    /// * `base_url` - Optional custom base URL (defaults to `https://api.akahu.io/v1`)
    pub fn new<T: Into<AppToken>>(
        client: reqwest::Client,
        app_id_token: T,
        base_url: Option<String>,
    ) -> Self {
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
