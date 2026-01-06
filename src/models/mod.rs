mod account;
mod identity;
mod me;
mod transaction;

pub use account::*;
pub use identity::*;
pub use me::*;
pub use transaction::*;

use serde::{Deserialize, Serialize};

use crate::Cursor;

// TODO: could we combine all three of these response types into one generic type?

/// Standard error response structure from Akahu API
///
/// All API errors follow this format with a success flag and message field.
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

/// Standard API response wrapper for a single item.
///
/// Most Akahu API endpoints that return a single resource wrap the response
/// in this format with a `success` field and the actual data in the `item` field.
///
/// # Example JSON
/// ```json
/// {
///   "success": true,
///   "item": { ... }
/// }
/// ```
///
/// [<https://developers.akahu.nz/docs/response-formatting>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ItemResponse<T> {
    /// Indicates if the request was successful.
    pub success: bool,

    /// The resource data.
    pub item: T,
}

/// Standard API response wrapper for a list of items.
///
/// Most Akahu API endpoints that return a list of resources wrap the response
/// in this format with a `success` field and the actual data in the `items` array.
///
/// # Example JSON
/// ```json
/// {
///   "success": true,
///   "items": [...]
/// }
/// ```
///
/// [<https://developers.akahu.nz/docs/response-formatting>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ListResponse<T> {
    /// Indicates if the request was successful.
    pub success: bool,

    /// The list of resources.
    pub items: Vec<T>,
}

#[derive(serde::Deserialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub items: Vec<T>,
    pub cursor: CursorObject,
}

/// Cursor for paginating through transaction results.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CursorObject {
    /// Cursor value to use for fetching the next page of results.
    pub next: Option<Cursor>,
}
