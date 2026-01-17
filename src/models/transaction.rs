//! Rust structs representing the Transaction Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/the-transaction-model).

use serde::{Deserialize, Serialize};

use crate::{AccountId, BankAccountNumber, CategoryId, ConnectionId, MerchantId, TransactionId};

/// A transaction is a record of money moving between two accounts. Akahu can
/// provide transaction data from connected accounts for all bank integrations
/// and a selection of non-bank integrations. See the [Supported
/// Integrations](https://developers.akahu.nz/docs/integrations) reference for
/// the full list of integrations that have transaction data available.
///
/// In addition to the basic transaction data that Akahu retrieves from the
/// connected institution (such as date, amount, description), Akahu enriches
/// transactions with merchant and categorisation data where possible. More
/// information on enrichment data is provided in detail in this document.
///
/// Transaction data is only available to apps with the TRANSACTIONS scope. In
/// addition, further permissions are required to access enriched transaction
/// data. Personal apps have full access to enriched transactions by default. If
/// you have a full Akahu app and would like access to transaction data, get in
/// touch via [hello@akahu.nz](mailto:hello@akahu.nz) or our [Slack
/// workspace](http://slack.akahu.io/).
///
/// See our [Accessing Transactional
/// Data](https://developers.akahu.nz/docs/accessing-transactional-data/) guide
/// to learn how to retrieve transactions from Akahu's API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// The `id` key is a unique identifier for the transaction in the Akahu
    /// system. It is always be prefixed by trans_ so that you can tell that it
    /// refers to a transaction.
    #[serde(rename = "_id")]
    pub id: TransactionId,

    /// The `account` key indicates which account this transaction belongs to.
    /// See our guide to Accessing Account Data to learn how to get this
    /// account, and our Account Model docs to learn more about accounts.
    #[serde(rename = "_account")]
    pub account: AccountId,

    /// This is the ID of provider that Akahu has retrieved this transaction
    /// from. You can get a list of connections from our
    /// [/connections](https://developers.akahu.nz/reference/get_connections)
    /// endpoint.
    #[serde(rename = "_connection")]
    pub connection: ConnectionId,

    /// The time that Akahu first saw this transaction (as an ISO 8601
    /// timestamp). This is unrelated to the transaction date (when the
    /// transaction occurred) because Akahu may have retrieved an old
    /// transaction.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The date that the transaction was posted with the account holder, as an
    /// ISO 8601 timestamp. In many cases this will only be accurate to the day,
    /// due to the level of detail provided by the bank.
    pub date: chrono::DateTime<chrono::Utc>,

    /// The transacton description as provided by the bank. Some minor cleanup
    /// is done by Akahu (such as whitespace normalisation), but this value is
    /// otherwise direct from the bank.
    pub description: String,

    /// The amount of money that was moved by this transaction.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// If available, the account balance immediately after this transaction was
    /// made. This value is direct from the bank and not modified by Akahu.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::arbitrary_precision_option"
    )]
    pub balance: Option<rust_decimal::Decimal>,

    /// What sort of transaction this is. Akahu tries to find a specific transaction
    /// type, falling back to "CREDIT" or "DEBIT" if nothing else is available.
    ///
    /// [<https://developers.akahu.nz/docs/the-transaction-model#type>]
    #[serde(rename = "type")]
    pub kind: TransactionKind,

    /// This is data added by the Akahu enrichment engine. You must have
    /// additional permissions to view this data.
    ///
    /// [<https://developers.akahu.nz/docs/the-transaction-model#enriched-transaction-data>]
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub enriched_data: Option<EnrichedTransactionData>,
}

/// What sort of transaction this is. Akahu tries to find a specific transaction
/// type, falling back to "CREDIT" or "DEBIT" if nothing else is available.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#type>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TransactionKind {
    /// Money has entered the account.
    #[serde(rename = "CREDIT")]
    Credit,
    /// Money has left the account.
    #[serde(rename = "DEBIT")]
    Debit,
    /// A payment to an external account.
    #[serde(rename = "PAYMENT")]
    Payment,
    /// A transfer between accounts that are associated with the same credentials.
    #[serde(rename = "TRANSFER")]
    Transfer,
    /// An automatic payment.
    #[serde(rename = "STANDING ORDER")]
    StandingOrder,
    /// A payment made via the EFTPOS system.
    #[serde(rename = "EFTPOS")]
    Eftpos,
    /// An interest payment from the account provider.
    #[serde(rename = "INTEREST")]
    Interest,
    /// A fee from the account provider.
    #[serde(rename = "FEE")]
    Fee,
    /// A tax payment.
    #[serde(rename = "TAX")]
    Tax,
    /// A credit card payment.
    #[serde(rename = "CREDIT CARD")]
    CreditCard,
    /// A direct debit payment.
    #[serde(rename = "DIRECT DEBIT")]
    DirectDebit,
    /// A direct credit (someone paying into the account).
    #[serde(rename = "DIRECT CREDIT")]
    DirectCredit,
    /// An ATM deposit or withdrawal.
    #[serde(rename = "ATM")]
    Atm,
    /// A payment related to a loan.
    #[serde(rename = "LOAN")]
    Loan,
}

impl TransactionKind {
    /// Get the transaction kind as a string slice.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Credit => "CREDIT",
            Self::Debit => "DEBIT",
            Self::Payment => "PAYMENT",
            Self::Transfer => "TRANSFER",
            Self::StandingOrder => "STANDING ORDER",
            Self::Eftpos => "EFTPOS",
            Self::Interest => "INTEREST",
            Self::Fee => "FEE",
            Self::Tax => "TAX",
            Self::CreditCard => "CREDIT CARD",
            Self::DirectDebit => "DIRECT DEBIT",
            Self::DirectCredit => "DIRECT CREDIT",
            Self::Atm => "ATM",
            Self::Loan => "LOAN",
        }
    }

    /// Get the transaction kind as bytes.
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl std::str::FromStr for TransactionKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREDIT" => Ok(Self::Credit),
            "DEBIT" => Ok(Self::Debit),
            "PAYMENT" => Ok(Self::Payment),
            "TRANSFER" => Ok(Self::Transfer),
            "STANDING ORDER" => Ok(Self::StandingOrder),
            "EFTPOS" => Ok(Self::Eftpos),
            "INTEREST" => Ok(Self::Interest),
            "FEE" => Ok(Self::Fee),
            "TAX" => Ok(Self::Tax),
            "CREDIT CARD" => Ok(Self::CreditCard),
            "DIRECT DEBIT" => Ok(Self::DirectDebit),
            "DIRECT CREDIT" => Ok(Self::DirectCredit),
            "ATM" => Ok(Self::Atm),
            "LOAN" => Ok(Self::Loan),
            _ => Err(()),
        }
    }
}

impl std::convert::TryFrom<String> for TransactionKind {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<&str> for TransactionKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// This is data added by the Akahu enrichment engine. You must have additional
/// permissions to view this data.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#enriched-transaction-data>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct EnrichedTransactionData {
    /// Category information for this transaction
    pub category: TransactionCategory,
    /// Merchant information for this transaction
    pub merchant: TransactionMerchant,
}

/// Transaction category information from Akahu enrichment.
///
/// Categories are based on the New Zealand Financial Category Codes (NZFCC) standard.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransactionCategory {
    /// Unique category identifier
    #[serde(rename = "_id")]
    pub id: CategoryId,
    /// NZFCC category code
    pub name: nzfcc::NzfccCode,
    /// Category groupings
    pub groups: TransactionGroups,
}

/// Category groupings for different classification systems.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransactionGroups {
    /// Personal finance category group
    pub personal_finance: PersonalFinanceGroup,
    /// Other category groupings (future extension)
    #[serde(flatten)]
    pub other_groups: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Personal finance category group.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PersonalFinanceGroup {
    /// Category group identifier
    #[serde(rename = "_id")]
    pub id: CategoryId,
    /// Category group name
    pub name: nzfcc::CategoryGroup,
}

/// Akahu defines a merchant as the business who was party to this transaction.
/// For example, "The Warehouse" is a merchant.
///
/// Merchant data is provided as a name, an optional website, and a merchant
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#merchant>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransactionMerchant {
    /// A unique identifier for the merchant in the Akahu system.
    ///
    /// Will always be prefixed with `_merchant`.
    #[serde(rename = "_id")]
    pub id: MerchantId,
    /// The name of the merchant, for example "The Warehouse".
    pub name: String,
    /// The merchant's website, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<url::Url>,
}

/// This is other metadata that we extract from the transaction, including the
/// following fields (where possible).
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#meta>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransactionMeta {
    /// Fields that are entered when a payment is made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub particulars: Option<String>,

    /// Fields that are entered when a payment is made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Fields that are entered when a payment is made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// The formatted NZ bank account number of the other party to this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_account: Option<BankAccountNumber>,

    /// If this transaction was made in another currency, details about the currency conversion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion: Option<TransactionConversion>,

    /// If this transaction was made with a credit or debit card, we may be able to extract the
    /// card number used to make the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_suffix: Option<String>,

    /// URL of a .png image for this transaction. This is typically the logo of the transaction merchant.
    /// If no logo is available, a placeholder image is provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<url::Url>,
}

/// Details about a currency conversion for a transaction made in another currency.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TransactionConversion {
    /// The amount in the foreign currency.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,
    /// The currency code of the foreign currency (e.g. "GBP").
    pub currency: iso_currency::Currency,
    /// The conversion rate applied.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub rate: rust_decimal::Decimal,
}

/// A pending transaction that has not yet been settled.
///
/// Pending transactions are not stable - the date or description may change due to
/// the unreliable nature of underlying NZ bank data. They are not assigned unique
/// identifiers and are not enriched by Akahu.
///
/// [<https://developers.akahu.nz/docs/accessing-transactional-data#pending-transactions>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PendingTransaction {
    /// The account this pending transaction belongs to.
    #[serde(rename = "_account")]
    pub account: AccountId,

    /// This is the ID of provider that Akahu has retrieved this transaction from.
    #[serde(rename = "_connection")]
    pub connection: ConnectionId,

    /// The time that this pending transaction was last updated (as an ISO 8601 timestamp).
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// The date that the transaction was posted with the account holder, as an
    /// ISO 8601 timestamp. May change before the transaction settles.
    pub date: chrono::DateTime<chrono::Utc>,

    /// The transaction description as provided by the bank. May change before settlement.
    pub description: String,

    /// The amount of money that will be moved by this transaction.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: rust_decimal::Decimal,

    /// What sort of transaction this is.
    #[serde(rename = "type")]
    pub kind: TransactionKind,

    /// Additional metadata about the transaction.
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<TransactionMeta>,
}
