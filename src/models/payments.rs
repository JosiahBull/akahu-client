//! Rust structs representing the Payment Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/making-a-payment).

/// A payment object represents a payment initiated from a user's connected bank account
/// to another New Zealand bank account.
///
/// Payments go through various states during their lifecycle, from creation through
/// to final completion or failure.
///
/// [<https://developers.akahu.nz/docs/making-a-payment>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Payment {
    /// The unique payment identifier in the Akahu system.
    #[serde(rename = "_id")]
    pub id: String,

    /// Unique Akahu identifier in the format "akpxxxxxxxxx".
    pub sid: String,

    /// Current payment status.
    pub status: PaymentStatus,

    /// Source account ID (prefixed with `acc_`).
    pub from: String,

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
    pub timeline: Vec<PaymentTimelineEntry>,

    /// Indicates if payment processing is complete (no further status changes expected).
    pub final_: bool,

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
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
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
///
/// # Example
///
/// ```no_run
/// # use akahu_client::PaymentDestination;
/// let destination = PaymentDestination::builder()
///     .account_number("01-2345-6789012-34")
///     .name("John Doe")
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(String, into))]
pub struct PaymentDestination {
    /// NZ bank account number in format "01-2345-6789012-34".
    pub account_number: String,

    /// Destination account holder name.
    /// Limited to alphanumeric characters, spaces, hyphens, and underscores: `/[A-z0-9 \-_]/`
    pub name: String,
}

/// Additional metadata for payments including statement details.
///
/// These fields appear on bank statements for both the source and destination accounts.
///
/// # Example
///
/// ```no_run
/// # use akahu_client::{PaymentMetadata, PaymentStatementDetails};
/// let metadata = PaymentMetadata::builder()
///     .destination(PaymentStatementDetails::builder()
///         .reference("INV-12345")
///         .build())
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(PaymentStatementDetails, into))]
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
///
/// # Example
///
/// ```no_run
/// # use akahu_client::PaymentStatementDetails;
/// let details = PaymentStatementDetails::builder()
///     .particulars("Invoice")
///     .code("12345")
///     .reference("ACME-001")
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(String, into))]
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
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PaymentTimelineEntry {
    /// The status at this point in the timeline.
    pub status: PaymentStatus,

    /// When this status change occurred.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Request body for creating a standard payment.
///
/// [<https://developers.akahu.nz/reference/post_payments>]
///
/// # Example
///
/// ```no_run
/// # use akahu_client::{CreatePaymentRequest, PaymentDestination};
/// # use rust_decimal::Decimal;
/// let request = CreatePaymentRequest::builder()
///     .from("acc_123...")
///     .to(PaymentDestination::builder()
///         .account_number("01-2345-6789012-34")
///         .name("John Doe")
///         .build())
///     .amount(Decimal::new(5000, 2)) // $50.00
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(String, into), on(rust_decimal::Decimal, into), on(PaymentDestination, into))]
pub struct CreatePaymentRequest {
    /// Source account ID (prefixed with `acc_`).
    /// The account must have the `PAYMENT_FROM` attribute.
    pub from: String,

    /// Destination account details.
    pub to: PaymentDestination,

    /// Payment amount in New Zealand dollars.
    /// Maximum single payment: $100,000 (Akahu limit).
    /// Bank-specific limits may also apply.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// Optional payment metadata including statement details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaymentMetadata>,
}

/// Request body for creating an IRD (Inland Revenue Department) tax payment.
///
/// [<https://developers.akahu.nz/reference/post_payments-ird>]
///
/// # Example
///
/// ```no_run
/// # use akahu_client::CreateIrdPaymentRequest;
/// # use rust_decimal::Decimal;
/// let request = CreateIrdPaymentRequest::builder()
///     .from("acc_123...")
///     .amount(Decimal::new(15000, 2)) // $150.00
///     .build();
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, bon::Builder)]
#[builder(on(String, into), on(rust_decimal::Decimal, into))]
pub struct CreateIrdPaymentRequest {
    /// Source account ID (prefixed with `acc_`).
    /// The account must have the `PAYMENT_FROM` attribute.
    pub from: String,

    /// Payment amount in New Zealand dollars.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// Optional payment metadata including IRD-specific details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaymentMetadata>,
}

/// Query parameters for listing payments.
///
/// [<https://developers.akahu.nz/reference/get_payments>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct ListPaymentsQuery {
    /// Start of the time range (exclusive) in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// End of the time range (inclusive) in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Response wrapper for a list of payments.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PaymentsListResponse {
    /// Success indicator.
    pub success: bool,

    /// List of payments.
    pub items: Vec<Payment>,
}

/// Response wrapper for a single payment operation.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PaymentResponse {
    /// Success indicator.
    pub success: bool,

    /// The payment object.
    pub item: Payment,
}
