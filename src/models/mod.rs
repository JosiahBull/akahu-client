mod account;
mod auth;
mod category;
mod connection;
mod identity;
mod me;
mod payments;
mod scopes;
mod support;
mod transaction;
mod transfers;

pub use account::*;
pub use auth::*;
pub use category::*;
pub use connection::*;
pub use identity::*;
pub use me::*;
pub use payments::*;
pub use scopes::*;
pub use support::*;
pub use transaction::*;
pub use transfers::*;

/// Standard error response structure from Akahu API
///
/// All API errors follow this format with a success flag and message field.
#[derive(Debug, serde::Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}
