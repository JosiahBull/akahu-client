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
//! use akahu_client::{AkahuClient, UserToken};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client with your app token
//! let client = AkahuClient::new(
//!     reqwest::Client::new(),
//!     "app_token_...".to_string(),
//!     None
//! );
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
//!
//! ## Unknown values from Akahu
//!
//! Every enum in this crate that mirrors a set of strings *Akahu* chooses — a transaction
//! `type`, an account `type`, `status`, `attribute` or `connection_type`, an identity `type`
//! or match result — has an `Unknown` catch-all variant and is `#[non_exhaustive]`.
//!
//! This matters more than it looks, because the blast radius of one unrecognised string is
//! not one field. Akahu documents a transaction's `type` as best-effort — it "tries to find a
//! specific transaction type, falling back to `CREDIT` or `DEBIT`" — so the vocabulary is
//! Akahu's to extend whenever it likes. A page is deserialised as a whole, so a value this
//! crate has never heard of used to fail all 100 transactions it arrived with, and a caller
//! that advances its sync cursor only on success would then refetch the same window, fail on
//! it again, and import nothing for that account until this crate was republished. An
//! `Unknown` variant turns an indefinite outage into one uninteresting value.
//!
//! `Unknown` does **not** carry the string it stood in for. It is a bare unit variant: that
//! is what serde's [`other`](https://serde.rs/variant-attrs.html#other) attribute supports
//! without hand-writing `Deserialize` for each of these enums, and a caller that needs the
//! original value still has the response it came from. Serialising an `Unknown` writes the
//! literal `UNKNOWN` (or `unknown`, per that enum's own casing), so a round-trip through this
//! crate is explicit about having lost the value rather than inventing one.
//!
//! `FromStr` follows the wire format rather than contradicting it: parsing an unrecognised
//! string yields `Unknown` instead of an error, so a value stored as text and read back
//! behaves the same way it did on arrival.
//!
//! `#[non_exhaustive]` is the other half of the deal. Matching one of these enums from
//! outside this crate needs a wildcard arm, which is what makes *naming* a new variant later
//! — once Akahu documents what it means — a non-breaking change.

#![warn(missing_docs)]

mod bank_account_number;
mod client;
mod error;
mod models;
mod serde;
mod types;

pub use bank_account_number::*;
pub use client::{AkahuClient, DEFAULT_MAX_RESPONSE_BYTES};
pub use error::{AkahuError, ResponseBody};
pub use models::*;
pub(crate) use serde::*;
pub use types::*;
