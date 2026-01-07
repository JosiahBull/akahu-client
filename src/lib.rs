//! # akahu-client
//!
//! A non-official Rust client library for the [Akahu API](https://www.akahu.nz/),
//! providing access to financial data aggregation services in New Zealand.
//!
//! ## Features
//!
//! - Fetch user accounts and account details
//! - Retrieve transactions with pagination support
//! - Access user identity and profile information
//! - Type-safe API with strongly-typed models
//! - Async/await support using tokio
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```no_run
//! use akahu_client::{AkahuClient, ReqwestClient, UserToken};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an HTTP client (using reqwest)
//! let http_client = ReqwestClient::new(reqwest::Client::new());
//!
//! // Create a client with your app token
//! let client = AkahuClient::new(
//!     http_client,
//!     "app_token_...".to_string(),
//!     None
//! );
//!
//! // Or use the convenience method when reqwest feature is enabled
//! let client = AkahuClient::with_reqwest("app_token_...");
//!
//! // Create a user token from OAuth flow
//! let user_token = UserToken::new("user_token_...".to_string());
//!
//! // Fetch accounts
//! let accounts = client.get_accounts(&user_token).await?;
//!
//! for account in accounts.items {
//!     println!("{}: {:?} - {:.2}",
//!         account.name,
//!         account.kind,
//!         account.balance.current
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Authentication
//!
//! The Akahu API requires two types of tokens:
//! - **App Token**: Identifies your application (obtained from Akahu dashboard)
//! - **User Token**: Identifies the user whose data you're accessing (obtained via OAuth flow)

#![warn(missing_docs)]

mod bank_account_number;
mod client;
mod error;
pub mod http;
mod models;
mod serde;
mod types;

pub use bank_account_number::*;
pub use client::AkahuClient;
pub use error::AkahuError;
#[cfg(feature = "reqwest")]
pub use http::ReqwestClient;
pub use models::*;
pub(crate) use serde::*;
pub use types::*;

// Rexport important types from external crates.
pub use chrono::{DateTime, Utc};
pub use nzfcc::*;
pub use url::Url;
