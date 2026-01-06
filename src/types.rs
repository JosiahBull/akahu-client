//! Type-safe wrappers for primitive types used throughout the Akahu API.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ============================================================================
// Macros for generating NewTypes
// ============================================================================

/// Macro for creating simple NewTypes without validation
macro_rules! newtype_string {
    ($(#[$attr:meta])* $vis:vis $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $vis struct $name(String);

        impl $name {
            /// Create a new instance
            pub fn new<T: Into<String>>(value: T) -> Self {
                Self(value.into())
            }

            /// Get the inner string value as a reference
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume and get the inner string value
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self::new(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::new(s)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

/// Macro for creating validated NewTypes with prefix checking
macro_rules! newtype_id {
    ($(#[$attr:meta])* $vis:vis $name:ident, $prefix:expr) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $name(String);

        impl $name {
            /// The expected prefix for this ID type
            pub const PREFIX: &'static str = $prefix;

            /// Create a new ID, validating the prefix
            pub fn new<T: Into<String>>(value: T) -> Result<Self, InvalidIdError> {
                let s = value.into();
                if !s.starts_with(Self::PREFIX) {
                    return Err(InvalidIdError {
                        type_name: stringify!($name),
                        expected_prefix: Self::PREFIX,
                        actual_value: s,
                    });
                }
                Ok(Self(s))
            }

            /// Create without validation (for deserialization from trusted API)
            pub(crate) fn new_unchecked<T: Into<String>>(value: T) -> Self {
                Self(value.into())
            }

            /// Get the inner string value as a reference
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume and get the inner string value
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // Custom serde implementation to use new_unchecked during deserialization
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self::new_unchecked(s))
            }
        }
    };
}

// ============================================================================
// Error Types
// ============================================================================

/// Error when an ID doesn't have the expected prefix
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid {type_name}: expected prefix '{expected_prefix}', got '{actual_value}'")]
pub struct InvalidIdError {
    /// The name of the ID type (e.g., "AccountId", "TransactionId")
    pub type_name: &'static str,
    /// The expected prefix for this ID type (e.g., "acc_", "trans_")
    pub expected_prefix: &'static str,
    /// The actual value that was provided
    pub actual_value: String,
}

/// Error when an email address is invalid
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid email address: '{0}'")]
pub struct InvalidEmailError(pub String);

// ============================================================================
// Authentication & Authorization Types
// ============================================================================

newtype_string!(
    /// User access token obtained through OAuth.
    ///
    /// This token is used to authenticate requests on behalf of a specific user.
    /// It grants access to the user's connected accounts and transaction data.
    pub UserToken
);

newtype_string!(
    /// Application ID token for authenticating your app with Akahu.
    ///
    /// This is your app's identifier, used in the `X-Akahu-Id` header.
    pub AppToken
);

newtype_string!(
    /// Application secret for app-scoped endpoints.
    ///
    /// Used in combination with the app token for HTTP Basic Authentication
    /// when accessing app-scoped endpoints like Categories and Connections.
    ///
    /// **Note**: Not available for Personal Apps.
    pub AppSecret
);

newtype_string!(
    /// OAuth client secret used during the token exchange flow.
    ///
    /// This may be the same as `AppSecret` depending on your app configuration.
    pub ClientSecret
);

newtype_string!(
    /// OAuth authorization code (short-lived, valid for ~60 seconds).
    ///
    /// Received from the OAuth flow and must be exchanged for a user access token
    /// within 60 seconds.
    pub AuthCode
);

newtype_string!(
    /// OAuth redirect URI used during the authorization flow.
    ///
    /// Must match exactly the URI configured in your Akahu app settings.
    pub RedirectUri
);

// ============================================================================
// Resource Identifiers with Validation
// ============================================================================

newtype_id!(
    /// Account identifier (always prefixed with `acc_`).
    ///
    /// Uniquely identifies a connected bank account within the Akahu system.
    pub AccountId,
    "acc_"
);

newtype_id!(
    /// Transaction identifier (always prefixed with `trans_`).
    ///
    /// Uniquely identifies a transaction within the Akahu system.
    pub TransactionId,
    "trans_"
);

newtype_id!(
    /// User identifier (always prefixed with `user_`).
    ///
    /// Uniquely identifies a user who has authorized your application.
    pub UserId,
    "user_"
);

newtype_id!(
    /// Transfer identifier (always prefixed with `transfer_`).
    ///
    /// Uniquely identifies a transfer between the user's accounts.
    pub TransferId,
    "transfer_"
);

newtype_id!(
    /// Payment identifier.
    ///
    /// Uniquely identifies a payment initiated through Akahu.
    pub PaymentId,
    "payment_"
);

newtype_id!(
    /// Connection identifier (always prefixed with `conn_`).
    ///
    /// Uniquely identifies a financial institution connection.
    pub ConnectionId,
    "conn_"
);

newtype_id!(
    /// Category identifier (always prefixed with `cat_`).
    ///
    /// Uniquely identifies an NZFCC category.
    pub CategoryId,
    "cat_"
);

newtype_id!(
    /// Merchant identifier (always prefixed with `_merchant`).
    ///
    /// Uniquely identifies a merchant in the Akahu enrichment system.
    pub MerchantId,
    "_merchant"
);

newtype_id!(
    /// Authorization identifier (always prefixed with `auth_`).
    ///
    /// Uniquely identifies an OAuth authorization.
    pub AuthorizationId,
    "auth_"
);

// ============================================================================
// Pagination & Query Types
// ============================================================================

newtype_string!(
    /// Pagination cursor token.
    ///
    /// Opaque token used to fetch the next page of results.
    /// Obtained from the `cursor.next` field in paginated responses.
    pub Cursor
);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_id_validation() {
        // Valid account ID
        AccountId::new("acc_123456").unwrap();

        // Invalid prefix
        AccountId::new("trans_123456").unwrap_err();
        AccountId::new("123456").unwrap_err();
    }

    #[test]
    fn test_transaction_id_validation() {
        // Valid transaction ID
        TransactionId::new("trans_abcdef123").unwrap();

        // Invalid prefix
        TransactionId::new("acc_123456").unwrap_err();
    }

    #[test]
    fn test_newtype_conversions() {
        let token = UserToken::new("test_token");
        assert_eq!(token.as_str(), "test_token");
        assert_eq!(&*token, "test_token"); // Via Deref
        assert_eq!(token.to_string(), "test_token");

        let token2: UserToken = "another_token".into();
        assert_eq!(token2.as_str(), "another_token");
    }
}
