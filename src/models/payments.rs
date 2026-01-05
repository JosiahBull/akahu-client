//! Rust structs representing the Payment Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/making-a-payment).

use serde::{Deserialize, Serialize};

use crate::{AccountId, BankAccountNumber, PaymentId};

/// A payment object represents a payment initiated from a user's connected bank account
/// to another New Zealand bank account.
///
/// Payments go through various states during their lifecycle, from creation through
/// to final completion or failure.
///
/// [<https://developers.akahu.nz/docs/making-a-payment>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Payment {
    /// The unique payment identifier in the Akahu system.
    #[serde(rename = "_id")]
    pub id: PaymentId,

    /// Unique Akahu identifier in the format "akpxxxxxxxxx".
    pub sid: String,

    /// Current payment status.
    pub status: PaymentStatus,

    /// Source account ID.
    pub from: AccountId,

    /// Destination account details.
    pub to: PaymentDestination,

    /// Payment amount in New Zealand dollars.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// Creation timestamp (ISO 8601).
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last update timestamp (ISO 8601).
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// Status change history with timestamps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timeline: Vec<PaymentTimelineEntry>,

    /// Indicates if payment processing is complete (no further status changes expected).
    #[serde(rename = "final")]
    pub complete: bool,

    /// Type of approval required when status is PENDING_APPROVAL.
    /// Either "BANK" (approval via bank) or "USER" (approval via MyAkahu).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_type: Option<String>,

    /// MyAkahu link for payment approval (when approval_type is "USER").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_url: Option<String>,

    /// When the payment arrived at the destination (optional, for arrival tracking).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Machine-readable error code (present for final statuses).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,

    /// Human-readable error explanation (present for final statuses).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,

    /// Additional payment metadata including statement details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaymentMetadata>,
}

/// Payment status values indicating the current state of the payment.
///
/// [<https://developers.akahu.nz/docs/making-a-payment>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    /// Payment is ready to be processed.
    Ready,
    /// Payment is awaiting approval (either from bank or user via MyAkahu).
    PendingApproval,
    /// Payment processing is paused.
    Paused,
    /// Payment was successfully sent (final status).
    Sent,
    /// Payment was declined by the bank or Akahu (final status).
    Declined,
    /// Akahu encountered an error processing the payment (final status).
    Error,
    /// Payment was cancelled by user, app, or system (final status).
    Cancelled,
}

/// Destination account details for a payment.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PaymentDestination {
    /// NZ bank account number in format "01-2345-6789012-34".
    pub account_number: BankAccountNumber,

    /// Destination account holder name.
    /// Limited to alphanumeric characters, spaces, hyphens, and underscores: `/[A-z0-9 \-_]/`
    pub name: String,
}

/// Additional metadata for payments including statement details.
///
/// These fields appear on bank statements for both the source and destination accounts.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PaymentMetadata {
    /// Statement details for the source account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentStatementDetails>,

    /// Statement details for the destination account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<PaymentStatementDetails>,
}

/// Statement details (Particulars, Code, Reference) for payment metadata.
///
/// Each field is limited to 12 characters: `/[A-z0-9 \-_]{0,12}/`
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PaymentStatementDetails {
    /// Particulars field on the bank statement (max 12 characters).
    /// Note: The payer's particulars field is reserved by Akahu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub particulars: Option<String>,

    /// Code field on the bank statement (max 12 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Reference field on the bank statement (max 12 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// An entry in the payment timeline showing a status change.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PaymentTimelineEntry {
    /// The status at this point in the timeline.
    pub status: PaymentStatus,

    /// When this status change occurred.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
