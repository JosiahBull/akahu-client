//! Akahu API client implementation.
//!
//! This module contains the main `AkahuClient` struct and all endpoint implementations,
//! organized by resource type for better maintainability.

use crate::{AppSecret, AppToken};

const DEFAULT_BASE_URL: &str = "https://api.akahu.io/v1";

/// The main Akahu API client.
///
/// Use the builder pattern to construct a new client.
pub struct AkahuClient {
    pub(super) client: reqwest::Client,
    pub(super) app_id_token: AppToken,
    pub(super) app_secret: Option<AppSecret>,
    pub(super) base_url: String,
}

#[bon::bon]
impl AkahuClient {
    /// Create a new Akahu client.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use for requests
    /// * `app_id_token` - Your Akahu application ID token
    /// * `base_url` - Optional custom base URL (defaults to `https://api.akahu.io/v1`)
    #[builder]
    pub fn new(
        client: reqwest::Client,
        app_id_token: impl Into<AppToken>,
        #[builder(default = DEFAULT_BASE_URL.to_string())] base_url: String,
    ) -> Self {
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
    ///
    /// **Note:** App-scoped endpoints are not available for Personal Apps.
    pub fn with_app_secret(mut self, app_secret: impl Into<AppSecret>) -> Self {
        self.app_secret = Some(app_secret.into());
        self
    }
}

// Core helper methods (headers, error handling, request execution)
mod core;

// Endpoint implementations - each file adds impl AkahuClient blocks
mod accounts;
mod auth;
mod categories;
mod connections;
mod me;
mod payments;
mod refresh;
mod transactions;
mod transfers;
