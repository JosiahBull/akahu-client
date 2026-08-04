//! Akahu API client implementation.

mod accounts;
mod core;
mod me;
mod refresh;
mod transactions;

use crate::{AppSecret, AppToken};

/// Default base URL for the Akahu API
const DEFAULT_BASE_URL: &str = "https://api.akahu.io/v1";

/// Default ceiling on how many bytes of a response body the client will buffer, before it
/// gives up with [`AkahuError::ResponseTooLarge`].
///
/// A response has to be buffered whole before it can be deserialised, and the timeout on the
/// `reqwest::Client` you hand to [`AkahuClient::new`] bounds how *long* that read may take,
/// not how big it may get: a 6-second budget on a gigabit link is several hundred megabytes.
/// Something has to bound the bytes, and only this crate is in a position to — it owns the
/// body read.
///
/// 8 MiB is picked to be uninteresting. The largest response Akahu documents is a page of
/// 100 transactions, which is well under a megabyte even with full enrichment on every
/// item, so the ceiling sits about an order of magnitude above anything real while still
/// being small enough that a runaway response fails fast instead of being paged into the
/// process. Raise it with [`AkahuClient::with_max_response_bytes`] if you have a legitimate
/// response that doesn't fit.
///
/// [`AkahuError::ResponseTooLarge`]: crate::AkahuError::ResponseTooLarge
pub const DEFAULT_MAX_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

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
    /// Ceiling on the number of body bytes any one response may be read into memory.
    max_response_bytes: u64,
}

impl AkahuClient {
    /// Create a new Akahu client.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use for requests
    /// * `app_id_token` - Your Akahu application ID token
    /// * `base_url` - Optional custom base URL (defaults to `https://api.akahu.io/v1`)
    ///
    /// Response bodies are capped at [`DEFAULT_MAX_RESPONSE_BYTES`]; use
    /// [`Self::with_max_response_bytes`] to change that.
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
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
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

    /// Set the ceiling on how many body bytes any one response may be read into memory.
    ///
    /// Defaults to [`DEFAULT_MAX_RESPONSE_BYTES`], which explains the reasoning behind the
    /// number. A response over the ceiling fails with
    /// [`AkahuError::ResponseTooLarge`](crate::AkahuError::ResponseTooLarge) — before a byte
    /// of the body is read when it declared an oversized `Content-Length`, and part-way
    /// through the read otherwise.
    pub const fn with_max_response_bytes(mut self, max_response_bytes: u64) -> Self {
        self.max_response_bytes = max_response_bytes;
        self
    }
}
