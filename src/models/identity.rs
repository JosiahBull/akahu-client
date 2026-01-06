//! Identity verification and party information models.
//!
//! Types for working with identity verification, party data, and address information.

use serde::{Deserialize, Serialize};

use crate::{BankAccountNumber, ConnectionId, space_separated_strings_as_vec};

/// Status of an identity verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentityStatus {
    /// Identity verification is still being processed
    Processing,
    /// Identity verification is complete
    Complete,
    /// Identity verification encountered an error
    Error,
}

/// Identity item containing account holder information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Identity {
    /// Account holder's name
    pub name: String,

    /// New Zealand bank account number in standard format (00-0000-0000000-00)
    pub formatted_account: BankAccountNumber,

    /// Reserved metadata object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Address information from financial institution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address {
    /// Type of address
    #[serde(rename = "type")]
    pub kind: AddressKind,

    /// Raw address string as provided by the bank
    pub value: String,

    /// Parsed and formatted address string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_address: Option<String>,

    /// Google Places API identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,

    /// Structured address components
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<AddressComponents>,
}

/// Type of address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressKind {
    /// Residential address
    Residential,
    /// Postal address
    Postal,
    /// Unknown address type
    Unknown,
}

/// Structured address components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddressComponents {
    /// Street address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// Suburb name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suburb: Option<String>,

    /// City name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Region or state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Postal code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Country name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Account information from identity verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityAccount {
    /// Account nickname or product name (e.g., "Spending", "Everyday")
    pub name: String,

    /// Account number in NZ format or masked identifier
    pub account_number: BankAccountNumber,

    /// Account holder name as displayed by the bank
    pub holder: String,

    /// Whether there are additional unlisted joint account holders
    pub has_unlisted_holders: bool,

    /// Optional address string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Bank/institution name
    pub bank: String,

    /// Optional branch information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchInfo>,
}

/// Bank branch information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchInfo {
    /// Unique Akahu ID beginning with `bank_branch_`
    #[serde(rename = "_id")]
    pub id: String,

    /// Descriptive name of the branch
    pub description: String,

    /// Phone number in E.164 format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Branch address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Information about the institution connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentitySource {
    /// Akahu Connection ID beginning with `conn_`
    #[serde(rename = "_id")]
    pub id: ConnectionId,
}

/// OAuth profile information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityProfile {
    /// Profile ID beginning with `profile_`
    #[serde(rename = "_id")]
    pub id: String,
}

/// Request to verify a name
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyNameRequest {
    /// Family name (surname) - required
    pub family_name: String,

    /// Given name (first name) - optional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// Middle name(s) - optional
    /// If multiple middle names, separate with spaces
    #[serde(
        rename = "middle_name",
        default,
        skip_serializing_if = "Option::is_none",
        with = "space_separated_strings_as_vec"
    )]
    pub middle_names: Option<Vec<String>>,
}

/// Response from name verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyNameResponse {
    /// Whether the verification was successful
    pub success: bool,

    /// Verification details
    pub item: VerifyNameItem,
}

/// Verification details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyNameItem {
    /// Array of verification sources (empty if no matches)
    pub sources: Vec<VerificationSource>,

    /// Echo of the input parameters
    pub name: VerifyNameRequest,
}

/// A single verification source result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerificationSource {
    /// Type of verification source
    #[serde(rename = "type")]
    pub source_type: VerificationSourceType,

    /// Source-specific metadata
    pub meta: serde_json::Value,

    /// Match result (only present if matched)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,

    /// Boolean flags indicating which name components matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<NameVerification>,
}

/// Type of verification source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationSourceType {
    /// Bank account holder name
    HolderName,
    /// Party name from financial institution
    PartyName,
}

/// Match result from verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchResult {
    /// All supplied parameters match the verification source
    Match,
    /// Family name matches but other supplied parameters don't
    PartialMatch,
}

/// Boolean flags for name component verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NameVerification {
    /// Whether family name matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<bool>,

    /// Whether given name matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<bool>,

    /// Whether middle name matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<bool>,

    /// Whether middle initial matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<bool>,

    /// Whether given initial matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_initial: Option<bool>,
}

/// Party information from enduring access
///
/// Contains customer profile information from financial institutions.
/// This is returned from the GET /parties endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Party {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: String,

    /// Party name
    pub name: String,

    /// Email address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Addresses associated with this party
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,

    /// Tax identification number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,

    /// Additional metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}
