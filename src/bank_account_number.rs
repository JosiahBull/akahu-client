use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

/// Error when a bank account number is invalid
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid NZ bank account number: '{0}' (expected format: XX-XXXX-XXXXXXX-XXX)")]
pub struct InvalidBankAccountError(pub String);

/// Represents the specific Bank/Financial Institution based on the 2-digit prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum BankPrefix {
    Anz = 1,
    Bnz = 2,
    Westpac = 3,
    AnzWise = 4,
    ChinaConstruction = 5,
    AnzNational = 6,
    Nab = 8,
    Icbc = 10,
    AnzPostBank = 11,
    Asb = 12,
    WestpacTrust = 13,
    WestpacOtago = 14,
    Tsb = 15,
    WestpacSouthland = 16,
    WestpacBop = 17,
    WestpacCanterbury = 18,
    WestpacWaikato = 19,
    WestpacWellington = 20,
    WestpacWestland = 21,
    WestpacSouthCant = 22,
    WestpacAuckland = 23,
    AsbPartner = 24,
    AnzPartner = 25,
    Hsbc = 30,
    Citibank = 31,
    Kiwibank = 38,
    BankOfChina = 88,
}

impl BankPrefix {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Anz => "01",
            Self::Bnz => "02",
            Self::Westpac => "03",
            Self::AnzWise => "04",
            Self::ChinaConstruction => "05",
            Self::AnzNational => "06",
            Self::Nab => "08",
            Self::Icbc => "10",
            Self::AnzPostBank => "11",
            Self::Asb => "12",
            Self::WestpacTrust => "13",
            Self::WestpacOtago => "14",
            Self::Tsb => "15",
            Self::WestpacSouthland => "16",
            Self::WestpacBop => "17",
            Self::WestpacCanterbury => "18",
            Self::WestpacWaikato => "19",
            Self::WestpacWellington => "20",
            Self::WestpacWestland => "21",
            Self::WestpacSouthCant => "22",
            Self::WestpacAuckland => "23",
            Self::AsbPartner => "24",
            Self::AnzPartner => "25",
            Self::Hsbc => "30",
            Self::Citibank => "31",
            Self::Kiwibank => "38",
            Self::BankOfChina => "88",
        }
    }

    pub fn bank_name(&self) -> &'static str {
        match self {
            Self::Anz
            | Self::AnzNational
            | Self::AnzPostBank
            | Self::AnzWise
            | Self::AnzPartner => "ANZ",
            Self::Bnz | Self::Nab => "Bank of New Zealand",
            Self::Westpac
            | Self::WestpacTrust
            | Self::WestpacOtago
            | Self::WestpacSouthland
            | Self::WestpacBop
            | Self::WestpacCanterbury
            | Self::WestpacWaikato
            | Self::WestpacWellington
            | Self::WestpacWestland
            | Self::WestpacSouthCant
            | Self::WestpacAuckland => "Westpac",
            Self::Asb | Self::AsbPartner => "ASB",
            Self::Kiwibank => "Kiwibank",
            Self::Tsb => "TSB",
            Self::ChinaConstruction => "China Construction Bank",
            Self::Icbc => "ICBC",
            Self::Hsbc => "HSBC",
            Self::Citibank => "Citibank",
            Self::BankOfChina => "Bank of China",
        }
    }
}

impl FromStr for BankPrefix {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numeric_part = s.trim_start_matches('0');
        if numeric_part.is_empty() {
            return Err(());
        }
        let val = numeric_part.parse::<u8>().map_err(|_| ())?;
        Self::try_from(val)
    }
}

impl TryFrom<u8> for BankPrefix {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Anz),
            2 => Ok(Self::Bnz),
            3 => Ok(Self::Westpac),
            4 => Ok(Self::AnzWise),
            5 => Ok(Self::ChinaConstruction),
            6 => Ok(Self::AnzNational),
            8 => Ok(Self::Nab),
            10 => Ok(Self::Icbc),
            11 => Ok(Self::AnzPostBank),
            12 => Ok(Self::Asb),
            13 => Ok(Self::WestpacTrust),
            14 => Ok(Self::WestpacOtago),
            15 => Ok(Self::Tsb),
            16 => Ok(Self::WestpacSouthland),
            17 => Ok(Self::WestpacBop),
            18 => Ok(Self::WestpacCanterbury),
            19 => Ok(Self::WestpacWaikato),
            20 => Ok(Self::WestpacWellington),
            21 => Ok(Self::WestpacWestland),
            22 => Ok(Self::WestpacSouthCant),
            23 => Ok(Self::WestpacAuckland),
            24 => Ok(Self::AsbPartner),
            25 => Ok(Self::AnzPartner),
            30 => Ok(Self::Hsbc),
            31 => Ok(Self::Citibank),
            38 => Ok(Self::Kiwibank),
            88 => Ok(Self::BankOfChina),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BankAccountNumber(String);

impl BankAccountNumber {
    /// Create a new bank account number with format validation.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidBankAccountError> {
        let s = value.into();
        let validate_parts = |parts: Vec<&str>| -> Result<(), ()> {
            if parts.len() != 4 {
                return Err(());
            }
            if BankPrefix::from_str(parts[0]).is_err() {
                return Err(());
            }
            if parts[1].len() != 4 || parts[2].len() != 7 || parts[3].len() != 3 {
                return Err(());
            }
            if !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
                return Err(());
            }
            Ok(())
        };

        if s.contains('-') {
            let parts: Vec<&str> = s.split('-').collect();
            if validate_parts(parts).is_err() {
                return Err(InvalidBankAccountError(s));
            }
            Ok(Self(s))
        } else {
            if s.len() != 16 || !s.chars().all(|c| c.is_ascii_digit()) {
                return Err(InvalidBankAccountError(s));
            }
            let parts = vec![&s[0..2], &s[2..6], &s[6..13], &s[13..16]];
            if validate_parts(parts).is_err() {
                return Err(InvalidBankAccountError(s));
            }
            let formatted = format!("{}-{}-{}-{}", &s[0..2], &s[2..6], &s[6..13], &s[13..16]);
            Ok(Self(formatted))
        }
    }

    /// Returns the Bank Prefix enum.
    pub fn prefix(&self) -> BankPrefix {
        BankPrefix::from_str(self.bank_code()).expect("Invalid prefix in stored account number")
    }

    /// Returns the 2-digit bank code string (e.g., "01").
    pub fn bank_code(&self) -> &str {
        &self.0[0..2]
    }

    /// Returns the 4-digit branch code string (e.g., "0123").
    pub fn branch_code(&self) -> &str {
        &self.0[3..7]
    }

    /// Returns the 7-digit account base number string (e.g., "0012345").
    pub fn account_number(&self) -> &str {
        &self.0[8..15]
    }

    /// Returns the 3-digit suffix string (e.g., "000").
    pub fn suffix(&self) -> &str {
        &self.0[16..19]
    }

    /// Returns the full string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for BankAccountNumber {
    type Err = InvalidBankAccountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for BankAccountNumber {
    type Error = InvalidBankAccountError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for BankAccountNumber {
    type Error = InvalidBankAccountError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for BankAccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for BankAccountNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for BankAccountNumber {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_extraction() {
        // Format: XX-XXXX-XXXXXXX-XXX
        //         01-2345-6789012-000
        let raw = "01-2345-6789012-000";
        let account = BankAccountNumber::new(raw).expect("Should be valid");

        assert_eq!(account.bank_code(), "01");
        assert_eq!(account.branch_code(), "2345");
        assert_eq!(account.account_number(), "6789012");
        assert_eq!(account.suffix(), "000");
    }

    #[test]
    fn test_component_extraction_from_unformatted() {
        // Unformatted input should result in correctly formatted output and extraction
        let raw = "3890000000000123"; // Kiwibank
        let account = BankAccountNumber::new(raw).expect("Should be valid");

        // Internal format should be 38-9000-0000000-123
        assert_eq!(account.as_str(), "38-9000-0000000-123");

        assert_eq!(account.prefix(), BankPrefix::Kiwibank);
        assert_eq!(account.bank_code(), "38");
        assert_eq!(account.branch_code(), "9000");
        assert_eq!(account.account_number(), "0000000");
        assert_eq!(account.suffix(), "123");
    }

    #[test]
    fn test_extraction_integrity() {
        // Ensure that reconstructing the string from components matches the original
        let account = BankAccountNumber::new("12-3456-7890123-001").unwrap();

        let reconstructed = format!(
            "{}-{}-{}-{}",
            account.bank_code(),
            account.branch_code(),
            account.account_number(),
            account.suffix()
        );

        assert_eq!(account.as_str(), reconstructed);
    }
}
