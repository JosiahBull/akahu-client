use serde::{Deserialize, Serialize};

// ==================== One-Off Identity Result ====================

/// One-off identity verification result
///
/// Contains identity data retrieved from financial institutions for verification purposes.
/// This is returned from the GET /identity/{id} endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResult {
    /// Unique identifier prefixed with `id_`
    #[serde(rename = "_id")]
    pub id: String,

    /// Current status of the identity verification
    pub status: IdentityStatus,

    /// When this identity result was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// When this identity result was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// When this identity result expires (typically 29 days after completion)
    pub expires_at: chrono::DateTime<chrono::Utc>,

    /// Array of identity items derived from account data
    pub identities: Vec<Identity>,

    /// Array of address items
    pub addresses: Vec<Address>,

    /// Array of account items
    pub accounts: Vec<IdentityAccount>,

    /// Information about the institution connection
    pub source: IdentitySource,

    /// Optional OAuth profile information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<IdentityProfile>,

    /// Optional array of non-fatal errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

/// Status of an identity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Account holder's name
    pub name: String,

    /// New Zealand bank account number in standard format (00-0000-0000000-00)
    pub formatted_account: String,

    /// Reserved metadata object
    pub meta: serde_json::Value,
}

/// Address information from financial institution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Type of address
    #[serde(rename = "type")]
    pub address_type: AddressType,

    /// Raw address string as provided by the bank
    pub value: String,

    /// Parsed and formatted address string (nullable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_address: Option<String>,

    /// Google Places API identifier (nullable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,

    /// Structured address components (nullable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<AddressComponents>,
}

/// Type of address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressType {
    /// Residential address
    Residential,
    /// Postal address
    Postal,
    /// Unknown address type
    Unknown,
}

/// Structured address components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressComponents {
    /// Street address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// Suburb
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suburb: Option<String>,

    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Account information from identity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAccount {
    /// Account nickname or product name (e.g., "Spending", "Everyday")
    pub name: String,

    /// Account number in NZ format or masked identifier
    pub account_number: String,

    /// Account holder name as displayed by the bank
    pub holder: String,

    /// Whether there are additional unlisted joint account holders
    pub has_unlisted_holders: bool,

    /// Optional address string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Bank/institution name
    pub bank: String,

    /// Optional branch information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<BranchInfo>,
}

/// Bank branch information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Unique Akahu ID beginning with `bank_branch_`
    #[serde(rename = "_id")]
    pub id: String,

    /// Descriptive name of the branch
    pub description: String,

    /// Phone number in E.164 format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Branch address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Information about the institution connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentitySource {
    /// Akahu Connection ID beginning with `conn_`
    #[serde(rename = "_id")]
    pub id: String,
}

/// OAuth profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProfile {
    /// Profile ID beginning with `profile_`
    #[serde(rename = "_id")]
    pub id: String,
}

// ==================== Name Verification ====================

/// Request to verify a name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNameRequest {
    /// Family name (surname) - required
    pub family_name: String,

    /// Given name (first name) - optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// Middle name(s) - optional
    /// If multiple middle names, separate with spaces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
}

/// Response from name verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNameResponse {
    /// Whether the verification was successful
    pub success: bool,

    /// Verification details
    pub item: VerifyNameItem,
}

/// Verification details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNameItem {
    /// Array of verification sources (empty if no matches)
    pub sources: Vec<VerificationSource>,

    /// Echo of the input parameters
    pub name: VerifyNameRequest,
}

/// A single verification source result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationSource {
    /// Type of verification source
    #[serde(rename = "type")]
    pub source_type: VerificationSourceType,

    /// Source-specific metadata
    pub meta: serde_json::Value,

    /// Match result (only present if matched)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,

    /// Boolean flags indicating which name components matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<NameVerification>,
}

/// Type of verification source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationSourceType {
    /// Bank account holder name
    HolderName,
    /// Party name from financial institution
    PartyName,
}

/// Match result from verification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchResult {
    /// All supplied parameters match the verification source
    Match,
    /// Family name matches but other supplied parameters don't
    PartialMatch,
}

/// Boolean flags for name component verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameVerification {
    /// Whether family name matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<bool>,

    /// Whether given name matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<bool>,

    /// Whether middle name matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<bool>,

    /// Whether middle initial matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<bool>,

    /// Whether given initial matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_initial: Option<bool>,
}

// ==================== Party (Enduring Identity) ====================

/// Party information from enduring access
///
/// Contains customer profile information from financial institutions.
/// This is returned from the GET /parties endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: String,

    /// Party name
    pub name: String,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Addresses associated with this party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,

    /// Tax identification number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}
