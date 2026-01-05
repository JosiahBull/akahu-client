//! Rust structs representing the Transfer Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/making-a-transfer).

/// A transfer is a money movement between two accounts belonging to the same user.
/// Transfers are initiated through Akahu and can be tracked through various status stages.
///
/// Transfer data is only available to apps with the appropriate permissions.
///
/// See our [Making a Transfer](https://developers.akahu.nz/docs/making-a-transfer) guide
/// to learn how to create and manage transfers through Akahu's API.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Transfer {
    /// The unique identifier for the transfer in the Akahu system.
    /// It is always prefixed by `transfer_` so that you can tell that it refers to a transfer.
    #[serde(rename = "_id")]
    pub id: String,

    /// The current status of the transfer.
    ///
    /// Normal statuses: READY, PENDING_APPROVAL, SENT
    /// Error statuses: DECLINED, ERROR, PAUSED, CANCELLED
    pub status: TransferStatus,

    /// The session identifier for this transfer.
    pub sid: String,

    /// The account ID of the source account (where money is coming from).
    /// Must be prefixed with `acc_`.
    pub from: String,

    /// The account ID of the destination account (where money is going to).
    /// Must be prefixed with `acc_`.
    pub to: String,

    /// The amount of money to transfer in dollars.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// The time that this transfer was created (as an ISO 8601 timestamp).
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The time that this transfer was last updated (as an ISO 8601 timestamp).
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// A historical record of status changes with timestamps.
    /// Each entry shows when the transfer transitioned to a particular status.
    pub timeline: Vec<TransferTimelineEntry>,

    /// Indicates if the transfer will no longer be updated.
    /// When `true`, the transfer has reached a final state and polling can cease.
    pub final_: bool,

    /// Additional details provided for ERROR or DECLINED status transfers.
    /// Provides context about why the transfer failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
}

/// The current status of a transfer.
///
/// Transfers progress through various statuses from creation to completion.
/// Some statuses indicate errors or exceptional conditions.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TransferTimelineEntry {
    /// The status that the transfer transitioned to.
    pub status: TransferStatus,

    /// The timestamp when this status change occurred (ISO 8601 format).
    pub time: chrono::DateTime<chrono::Utc>,
}

/// Parameters for creating a new transfer.
///
/// Use this struct when calling the POST /transfers endpoint to initiate
/// a transfer between two of the user's connected accounts.
///
/// # Example
///
/// ```no_run
/// # use akahu_client::TransferCreateParams;
/// # use rust_decimal::Decimal;
/// let params = TransferCreateParams::builder()
///     .from("acc_123...".to_string())
///     .to("acc_456...".to_string())
///     .amount(Decimal::new(10000, 2)) // $100.00
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(String, into), on(rust_decimal::Decimal, into))]
pub struct TransferCreateParams {
    /// The account ID of the destination account (where money is going to).
    /// Must be prefixed with `acc_`.
    pub to: String,

    /// The account ID of the source account (where money is coming from).
    /// Must be prefixed with `acc_`.
    pub from: String,

    /// The amount of money to transfer in dollars.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,
}

/// Query parameters for listing transfers.
///
/// Use this struct to filter transfers by date range when calling the GET /transfers endpoint.
///
/// # Example
///
/// ```no_run
/// # use akahu_client::TransferQueryParams;
/// let query = TransferQueryParams::builder()
///     .start(chrono::Utc::now() - chrono::Duration::days(7))
///     .end(chrono::Utc::now())
///     .build();
/// ```
#[derive(Debug, Default, Clone, bon::Builder)]
#[builder(on(chrono::DateTime<chrono::Utc>, into))]
pub struct TransferQueryParams {
    /// The start of the date range (exclusive).
    /// Defaults to 30 days ago if not specified.
    pub start: Option<chrono::DateTime<chrono::Utc>>,

    /// The end of the date range (inclusive).
    /// Defaults to now if not specified.
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}
