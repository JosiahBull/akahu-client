//! Identity verification and party information models.
//!
//! Types for working with identity verification, party data, and address information.

use serde::{Deserialize, Serialize};

use crate::{BankAccountNumber, ConnectionId, space_separated_strings_as_vec};

/// Status of an identity verification
///
/// A status this crate doesn't recognise becomes [`Self::Unknown`] rather than failing the
/// response — see the crate-level note on [unknown values](crate#unknown-values-from-akahu).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum IdentityStatus {
    /// Identity verification is still being processed
    Processing,
    /// Identity verification is complete
    Complete,
    /// Identity verification encountered an error
    Error,
    /// A status this crate doesn't recognise.
    ///
    /// Not the same as [`Self::Error`] — the verification may well have succeeded; only the
    /// word Akahu used for it is new. See the crate-level note on
    /// [unknown values](crate#unknown-values-from-akahu).
    #[serde(other)]
    Unknown,
}

impl IdentityStatus {
    /// Get the status as a string slice.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Processing => "PROCESSING",
            Self::Complete => "COMPLETE",
            Self::Error => "ERROR",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// Get the status as bytes.
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl std::str::FromStr for IdentityStatus {
    type Err = ();

    /// Parsing never fails: an unrecognised status becomes [`Self::Unknown`], matching how it
    /// deserialises on the wire. `Err` is kept for compatibility and is never returned.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PROCESSING" => Ok(Self::Processing),
            "COMPLETE" => Ok(Self::Complete),
            "ERROR" => Ok(Self::Error),
            _ => Ok(Self::Unknown),
        }
    }
}

impl std::convert::TryFrom<String> for IdentityStatus {
    type Error = ();
    fn try_from(value: String) -> Result<Self, ()> {
        value.parse()
    }
}

impl std::convert::TryFrom<&str> for IdentityStatus {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        value.parse()
    }
}

impl std::fmt::Display for IdentityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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
///
/// Akahu already documents an `UNKNOWN` address type, so [`Self::Unknown`] does double duty
/// here: it is both that value and the catch-all for an address type this crate doesn't
/// recognise. See the crate-level note on
/// [unknown values](crate#unknown-values-from-akahu).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AddressKind {
    /// Residential address
    Residential,
    /// Postal address
    Postal,
    /// Akahu's own `UNKNOWN` address type, and any address type this crate doesn't
    /// recognise — the two are indistinguishable here, and mean the same thing to a caller.
    #[serde(other)]
    Unknown,
}

impl AddressKind {
    /// Get the address kind as a string slice.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Residential => "RESIDENTIAL",
            Self::Postal => "POSTAL",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// Get the address kind as bytes.
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl std::str::FromStr for AddressKind {
    type Err = ();

    /// Parsing never fails: an unrecognised address type becomes [`Self::Unknown`], matching
    /// how it deserialises on the wire. `Err` is kept for compatibility and is never returned.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RESIDENTIAL" => Ok(Self::Residential),
            "POSTAL" => Ok(Self::Postal),
            _ => Ok(Self::Unknown),
        }
    }
}

impl std::convert::TryFrom<String> for AddressKind {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&str> for AddressKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for AddressKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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
///
/// A source this crate doesn't recognise becomes [`Self::Unknown`] rather than failing the
/// response — see the crate-level note on [unknown values](crate#unknown-values-from-akahu).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum VerificationSourceType {
    /// Bank account holder name
    HolderName,
    /// Party name from financial institution
    PartyName,
    /// A verification source this crate doesn't recognise. See the crate-level note on
    /// [unknown values](crate#unknown-values-from-akahu).
    #[serde(other)]
    Unknown,
}

impl VerificationSourceType {
    /// Get the verification source type as a string slice.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::HolderName => "HOLDER_NAME",
            Self::PartyName => "PARTY_NAME",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// Get the verification source type as bytes.
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl std::str::FromStr for VerificationSourceType {
    type Err = ();

    /// Parsing never fails: an unrecognised source becomes [`Self::Unknown`], matching how it
    /// deserialises on the wire. `Err` is kept for compatibility and is never returned.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HOLDER_NAME" => Ok(Self::HolderName),
            "PARTY_NAME" => Ok(Self::PartyName),
            _ => Ok(Self::Unknown),
        }
    }
}

impl std::convert::TryFrom<String> for VerificationSourceType {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&str> for VerificationSourceType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for VerificationSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Match result from verification
///
/// A result this crate doesn't recognise becomes [`Self::Unknown`] rather than failing the
/// response. Note the direction that fails in: an unrecognised result is not
/// [`Self::Match`], so a caller gating on a match still gates closed. See the crate-level
/// note on [unknown values](crate#unknown-values-from-akahu).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum MatchResult {
    /// All supplied parameters match the verification source
    Match,
    /// Family name matches but other supplied parameters don't
    PartialMatch,
    /// A match result this crate doesn't recognise. **Not** evidence of a match — see the
    /// note on the enum itself.
    #[serde(other)]
    Unknown,
}

impl MatchResult {
    /// Get the match result as a string slice.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Match => "MATCH",
            Self::PartialMatch => "PARTIAL_MATCH",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// Get the match result as bytes.
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl std::str::FromStr for MatchResult {
    type Err = ();

    /// Parsing never fails: an unrecognised result becomes [`Self::Unknown`], matching how it
    /// deserialises on the wire. `Err` is kept for compatibility and is never returned.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MATCH" => Ok(Self::Match),
            "PARTIAL_MATCH" => Ok(Self::PartialMatch),
            _ => Ok(Self::Unknown),
        }
    }
}

impl std::convert::TryFrom<String> for MatchResult {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&str> for MatchResult {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "tests need to unwrap to verify correctness"
)]
mod tests {
    use super::*;

    /// The identity endpoints carry the same exposure as accounts and transactions: their
    /// enums are Akahu's vocabulary, and a verification response is deserialised whole.
    #[test]
    fn a_verification_response_survives_unrecognised_enum_values() {
        let json = r#"{
            "success": true,
            "item": {
                "sources": [
                    {
                        "type": "IRD_NAME",
                        "meta": {},
                        "match_result": "FUZZY_MATCH"
                    },
                    {
                        "type": "HOLDER_NAME",
                        "meta": {},
                        "match_result": "MATCH"
                    }
                ],
                "name": { "family_name": "Bull" }
            }
        }"#;

        let response: VerifyNameResponse = serde_json::from_str(json).unwrap();
        let sources = response.item.sources;
        assert_eq!(sources.len(), 2, "the whole response must survive");

        let strange = sources.first().unwrap();
        assert_eq!(strange.source_type, VerificationSourceType::Unknown);
        // An unrecognised result must not read as a match — it fails closed.
        assert_eq!(strange.match_result, Some(MatchResult::Unknown));
        assert_ne!(strange.match_result, Some(MatchResult::Match));

        let known = sources.get(1).unwrap();
        assert_eq!(known.source_type, VerificationSourceType::HolderName);
        assert_eq!(known.match_result, Some(MatchResult::Match));
    }

    /// `AddressKind` already had an `Unknown` variant for Akahu's own `UNKNOWN` value; it now
    /// doubles as the catch-all, so both spellings land in the same place.
    #[test]
    fn an_unrecognised_address_type_joins_akahus_own_unknown() {
        let address = |kind: &str| -> Address {
            serde_json::from_str(&format!(r#"{{ "type": "{kind}", "value": "1 Queen St" }}"#))
                .unwrap()
        };
        assert_eq!(address("UNKNOWN").kind, AddressKind::Unknown);
        assert_eq!(address("BUSINESS").kind, AddressKind::Unknown);
        assert_eq!(address("RESIDENTIAL").kind, AddressKind::Residential);
    }

    #[test]
    fn an_unrecognised_identity_status_deserialises_to_unknown() {
        assert_eq!(
            serde_json::from_str::<IdentityStatus>("\"QUEUED\"").unwrap(),
            IdentityStatus::Unknown
        );
        // Not conflated with the ERROR status: the verification may well have worked.
        assert_ne!(
            serde_json::from_str::<IdentityStatus>("\"QUEUED\"").unwrap(),
            IdentityStatus::Error
        );
    }

    /// `as_str`/`FromStr`/`Serialize` have to agree about `Unknown`, and the known values must
    /// still round-trip rather than being swallowed by the catch-all.
    #[test]
    fn unknown_is_consistent_across_the_identity_enums() {
        assert_eq!(IdentityStatus::Unknown.as_str(), "UNKNOWN");
        assert_eq!(
            "QUEUED".parse::<IdentityStatus>().unwrap(),
            IdentityStatus::Unknown
        );
        assert_eq!(AddressKind::Unknown.as_str(), "UNKNOWN");
        assert_eq!(
            "BUSINESS".parse::<AddressKind>().unwrap(),
            AddressKind::Unknown
        );
        assert_eq!(VerificationSourceType::Unknown.as_str(), "UNKNOWN");
        assert_eq!(
            "IRD_NAME".parse::<VerificationSourceType>().unwrap(),
            VerificationSourceType::Unknown
        );
        assert_eq!(MatchResult::Unknown.as_str(), "UNKNOWN");
        assert_eq!(
            "FUZZY_MATCH".parse::<MatchResult>().unwrap(),
            MatchResult::Unknown
        );

        for status in [
            IdentityStatus::Processing,
            IdentityStatus::Complete,
            IdentityStatus::Error,
        ] {
            let wire = serde_json::to_string(&status).unwrap();
            assert_eq!(wire, format!("\"{}\"", status.as_str()));
            assert_eq!(
                serde_json::from_str::<IdentityStatus>(&wire).unwrap(),
                status,
                "{status} did not survive a round-trip"
            );
        }
        for kind in [AddressKind::Residential, AddressKind::Postal] {
            let wire = serde_json::to_string(&kind).unwrap();
            assert_eq!(wire, format!("\"{}\"", kind.as_str()));
            assert_eq!(
                serde_json::from_str::<AddressKind>(&wire).unwrap(),
                kind,
                "{kind} did not survive a round-trip"
            );
        }
        for source in [
            VerificationSourceType::HolderName,
            VerificationSourceType::PartyName,
        ] {
            let wire = serde_json::to_string(&source).unwrap();
            assert_eq!(wire, format!("\"{}\"", source.as_str()));
            assert_eq!(
                serde_json::from_str::<VerificationSourceType>(&wire).unwrap(),
                source,
                "{source} did not survive a round-trip"
            );
        }
        for result in [MatchResult::Match, MatchResult::PartialMatch] {
            let wire = serde_json::to_string(&result).unwrap();
            assert_eq!(wire, format!("\"{}\"", result.as_str()));
            assert_eq!(
                serde_json::from_str::<MatchResult>(&wire).unwrap(),
                result,
                "{result} did not survive a round-trip"
            );
        }
    }
}
