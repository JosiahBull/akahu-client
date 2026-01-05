//! Category models for the Akahu API.
//!
//! Categories use the New Zealand Financial Category Codes (NZFCC) standard.
//! See <https://nzfcc.org> for more information.

/// A transaction category using NZFCC (New Zealand Financial Category Codes).
///
/// [<https://developers.akahu.nz/reference/get_categories>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Category {
    /// A unique identifier for the NZFCC category
    #[serde(rename = "_id")]
    pub id: String,

    /// The human-readable category name (e.g., "Cafes & Restaurants")
    pub name: nzfcc::NzfccCode,

    /// Category groupings at different granularity levels
    pub groups: CategoryGroups,
}

/// Category groupings at different granularity levels.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#category>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct CategoryGroups {
    /// The personal finance category group (always present)
    pub personal_finance: CategoryGroup,

    /// Additional custom groupings configured per application
    #[serde(flatten)]
    pub other_groups: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// A category group with identifier and name.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#category>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct CategoryGroup {
    /// Identifier for this specific grouping
    #[serde(rename = "_id")]
    pub id: String,

    /// The group's display name (e.g., "Lifestyle")
    pub name: nzfcc::CategoryGroup,
}
