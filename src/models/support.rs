//! Support-related models for the Akahu API.
//!
//! These models are used to report issues with transactions, including duplicates,
//! enrichment errors, and enrichment suggestions.

/// Request body for reporting a transaction issue.
///
/// This is used with the POST /support/transaction/{id} endpoint to report
/// issues with transactions, including duplicates, enrichment errors, and enrichment suggestions.
///
/// [<https://developers.akahu.nz/reference/post_support-transaction-id>]
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TransactionSupportRequest {
    /// Report a duplicate transaction
    #[serde(rename = "DUPLICATE")]
    Duplicate {
        /// The ID of the duplicate transaction
        other_id: String,
    },

    /// Report an enrichment error
    #[serde(rename = "ENRICHMENT_ERROR")]
    EnrichmentError {
        /// Dot-separated paths to incorrect values (e.g., "merchant.name")
        fields: Vec<String>,
        /// Explanation of the error
        comment: String,
    },

    /// Suggest an enrichment improvement
    #[serde(rename = "ENRICHMENT_SUGGESTION")]
    EnrichmentSuggestion {
        /// Description of suggested enrichment
        comment: String,
    },
}

/// Response from a transaction support request.
///
/// [<https://developers.akahu.nz/reference/post_support-transaction-id>]
#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct TransactionSupportResponse {
    /// Indicates if the request was successful
    pub success: bool,
}
