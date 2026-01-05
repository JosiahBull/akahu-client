//! Rust structs representing the Transaction Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/the-transaction-model).

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
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Transaction {
    /// The `id` key is a unique identifier for the transaction in the Akahu
    /// system. It is always be prefixed by trans_ so that you can tell that it
    /// refers to a transaction.
    #[serde(rename = "_id")]
    id: String,

    /// The `account` key indicates which account this transaction belongs to.
    /// See our guide to Accessing Account Data to learn how to get this
    /// account, and our Account Model docs to learn more about accounts.
    #[serde(rename = "_account")]
    account: String,

    /// This is the ID of provider that Akahu has retrieved this transaction
    /// from. You can get a list of connections from our
    /// [/connections](https://developers.akahu.nz/reference/get_connections)
    /// endpoint.
    #[serde(rename = "_connection")]
    connection: String,

    /// The time that Akahu first saw this transaction (as an ISO 8601
    /// timestamp). This is unrelated to the transaction date (when the
    /// transaction occurred) because Akahu may have retrieved an old
    /// transaction.
    created_at: chrono::DateTime<chrono::Utc>,

    /// The date that the transaction was posted with the account holder, as an
    /// ISO 8601 timestamp. In many cases this will only be accurate to the day,
    /// due to the level of detail provided by the bank.
    date: chrono::DateTime<chrono::Utc>,

    /// The transacton description as provided by the bank. Some minor cleanup
    /// is done by Akahu (such as whitespace normalisation), but this value is
    /// otherwise direct from the bank.
    description: String,

    /// The amount of money that was moved by this transaction.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    amount: rust_decimal::Decimal,

    /// If available, the account balance immediately after this transaction was
    /// made. This value is direct from the bank and not modified by Akahu.
    #[serde(with = "rust_decimal::serde::arbitrary_precision_option")]
    balance: Option<rust_decimal::Decimal>,

    /// What sort of transaction this is. Akahu tries to find a specific transaction
    /// type, falling back to "CREDIT" or "DEBIT" if nothing else is available.
    ///
    /// [<https://developers.akahu.nz/docs/the-transaction-model#type>]
    #[serde(rename = "type")]
    kind: TransactionKind,

    /// This is data added by the Akahu enrichment engine. You must have
    /// additional permissions to view this data.
    ///
    /// [<https://developers.akahu.nz/docs/the-transaction-model#enriched-transaction-data>]
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    enriched_data: Option<EnrichedTransactionData>,
}

/// What sort of transaction this is. Akahu tries to find a specific transaction
/// type, falling back to "CREDIT" or "DEBIT" if nothing else is available.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#type>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
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

/// This is data added by the Akahu enrichment engine. You must have additional
/// permissions to view this data.
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#enriched-transaction-data>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct EnrichedTransactionData {
    category: TransactionCategory,
    merchant: TransactionMerchant,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct TransactionCategory {
    #[serde(rename = "_id")]
    id: String,
    name: nzfcc::NzfccCode,
    groups: TransactionGroups,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct TransactionGroups {
    personal_finance: PersonalFinanceGroup,
    #[serde(flatten)]
    other_groups: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct PersonalFinanceGroup {
    #[serde(rename = "_id")]
    id: String,
    name: nzfcc::CategoryGroup,
}

/// Akahu defines a merchant as the business who was party to this transaction.
/// For example, "The Warehouse" is a merchant.
///
/// Merchant data is provided as a name, an optional website, and a merchant
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#merchant>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct TransactionMerchant {
    /// A unique identifier for the merchant in the Akahu system.
    ///
    /// Will always be prefixed with `_merchant`.
    #[serde(rename = "_id")]
    id: String,
    /// The name of the merchant, for example "The Warehouse".
    name: String,
    /// The merchant's website, if available.
    website: Option<url::Url>,
}

/// This is other metadata that we extract from the transaction, including the
/// following fields (where possible).
///
/// [<https://developers.akahu.nz/docs/the-transaction-model#meta>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
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
    pub other_account: Option<String>,

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
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
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
