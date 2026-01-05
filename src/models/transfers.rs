//! Rust structs representing the Transfer Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/making-a-transfer).

use serde::{Deserialize, Serialize};

use crate::{AccountId, TransferId};

/// A transfer is a money movement between two accounts belonging to the same user.
/// Transfers are initiated through Akahu and can be tracked through various status stages.
///
/// Transfer data is only available to apps with the appropriate permissions.
///
/// See our [Making a Transfer](https://developers.akahu.nz/docs/making-a-transfer) guide
/// to learn how to create and manage transfers through Akahu's API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Transfer {
    /// The unique identifier for the transfer in the Akahu system.
    #[serde(rename = "_id")]
    pub id: TransferId,

    /// The current status of the transfer.
    ///
    /// Normal statuses: READY, PENDING_APPROVAL, SENT
    /// Error statuses: DECLINED, ERROR, PAUSED, CANCELLED
    pub status: TransferStatus,

    /// The session identifier for this transfer.
    pub sid: String,

    /// The account ID of the source account (where money is coming from).
    pub from: AccountId,

    /// The account ID of the destination account (where money is going to).
    pub to: AccountId,

    /// The amount of money to transfer in dollars.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// The time that this transfer was created (as an ISO 8601 timestamp).
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time that this transfer was last updated (as an ISO 8601 timestamp).
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// A historical record of status changes with timestamps.
    /// Each entry shows when the transfer transitioned to a particular status.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timeline: Vec<TransferTimelineEntry>,

    /// Indicates if the transfer will no longer be updated.
    /// When `true`, the transfer has reached a final state and polling can cease.
    #[serde(rename = "final")]
    pub complete: bool,

    /// Additional details provided for ERROR or DECLINED status transfers.
    /// Provides context about why the transfer failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
}

/// The current status of a transfer.
///
/// Transfers progress through various statuses from creation to completion.
/// Some statuses indicate errors or exceptional conditions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferStatus {
    /// Transfer is prepared and ready for processing.
    Ready,

    /// Transfer is awaiting user authorization/approval.
    PendingApproval,

    /// Transfer has been processed and should be visible in source account transactions.
    Sent,

    /// Transfer was rejected by the source bank.
    Declined,

    /// An Akahu system error occurred during processing.
    Error,

    /// Transfer is paused awaiting review (fraud/compliance).
    Paused,

    /// Transfer was cancelled by the user or system.
    Cancelled,
}

/// A timeline entry recording a status change for a transfer.
///
/// The timeline provides a historical record of all status transitions,
/// allowing you to track the progress of a transfer over time.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransferTimelineEntry {
    /// The status that the transfer transitioned to.
    pub status: TransferStatus,

    /// The timestamp when this status change occurred.
    pub time: chrono::DateTime<chrono::Utc>,
}
