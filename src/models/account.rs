//! Rust structs representing the Account Model Akahu uses, the documentation
//! for the Akahu model this is derived from is
//! [here](https://developers.akahu.nz/docs/the-account-model).

use serde::{Deserialize, Serialize};

use crate::{AccountId, AuthorizationId, BankAccountNumber};

/// An Akahu account is something that has a balance. Some connections (like
/// banks) have lots of accounts, while others (like KiwiSaver providers) may
/// only have one. Different types of accounts have different attributes and
/// abilities, which can get a bit confusing! The rest of this page should help
/// you figure everything out, from an account's provider to whether it can make
/// payments.
///
/// Keep in mind that we limit what information is available depending on your
/// app permissions. This is done in order to protect user privacy, however it
/// also means that some of the data here may not be visible to you.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Account {
    /// The `id` key is a unique identifier for the account in the Akahu system.
    ///
    /// It is always be prefixed by acc_ so that you can tell that it belongs to
    /// an account.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#_id>]
    #[serde(rename = "_id")]
    pub id: AccountId,

    /// The identifier of this account's predecessor.
    ///
    /// This attribute is only present if the account has been migrated to an
    /// official open banking connection from a classic Akahu connection.
    ///
    /// Read more about official open banking, and migrating to it
    /// [here](https://developers.akahu.nz/docs/official-open-banking).
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#_migrated>]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_migrated")]
    pub migrated: Option<String>,

    /// Financial accounts are connected to Akahu via an authorisation with the
    /// user's financial institution. Multiple accounts can be connected during
    /// a single authorisation, causing them to have the same authorisation
    /// identifier. This identifier can also be used to link a specific account
    /// to identity data for the
    /// [party](https://developers.akahu.nz/reference/get_parties) who completed
    /// the authorisation.
    ///
    /// This identifier can also be used to revoke access to all the accounts
    /// connected to that authorisation.
    ///
    /// For example, if you have 3 ANZ accounts, they will all have the same
    /// `authorisation`. Your ANZ accounts and your friend's ANZ accounts have
    /// different logins, so they will have a different `authorisation key`. The
    /// `authorisation` key is in no way derived or related to your login
    /// credentials - it's just a random ID.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#_authorisation>]
    #[serde(rename = "_authorisation")]
    pub authorisation: AuthorizationId,

    /// Deprecated: Please use `authorisation` instead.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#_credentials-deprecated>]
    #[deprecated(note = "Please use `authorisation` instead.")]
    #[serde(
        rename = "_credentials",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials: Option<AuthorizationId>,

    /// This is the name of the account. If the connection allows customisation,
    /// the name will be the custom name (or nickname), e.g. "Spending Account".
    /// Otherwise Akahu falls back to the product name, e.g. "Super Saver".
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#name>]
    pub name: String,

    /// This attribute indicates the status of Akahu's connection to this account.
    ///
    /// It is possible for Akahu to lose the ability to authenticate with a
    /// financial institution if the user revokes Akahu's access directly via
    /// their institution, or changes their login credentials, which in some
    /// cases can cause our long-lived access to be revoked.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#status>]
    pub status: Active,

    /// If the account has a well defined account number (eg. a bank account
    /// number, or credit card number) this will be defined here with a standard
    /// format across connections. This field will be the value undefined for
    /// accounts with KiwiSaver providers and investment platform accounts.
    ///
    /// For NZ banks, we use the common format 00-0000-0000000-00. For credit
    /// cards, we return a redacted card number 1234-****-****-1234 or
    /// ****-****-****-1234
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#formatted_account>]
    // TODO: could hyave a strongly defined type here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_acount: Option<String>,

    /// Akahu can refresh different parts of an account's data at different rates.
    /// The timestamps in the refreshed object tell you when that account data was
    /// last updated.
    ///
    /// When looking at a timestamp in here, you can think "Akahu's view of the
    /// account (balance/metadata/transactions) is up to date as of $TIME".
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#refreshed>]
    pub refreshed: RefreshDetails,

    /// The account balance.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#balance>]
    pub balance: BalanceDetails,

    /// What sort of account this is. Akahu provides specific bank account
    /// types, and falls back to more general types for other types of
    /// connection.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#type>]
    #[serde(rename = "type")]
    pub kind: BankAccountKind,

    /// The list of attributes indicates what abilities an account has.
    ///
    /// See [Attribute] for more information.
    ///
    /// [<https://developers.akahu.nz/docs/the-account-model#attributes>]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

/// This attribute indicates the status of Akahu's connection to this account.
///
/// It is possible for Akahu to lose the ability to authenticate with a
/// financial institution if the user revokes Akahu's access directly via
/// their institution, or changes their login credentials, which in some
/// cases can cause our long-lived access to be revoked.
///
/// [<https://developers.akahu.nz/docs/the-account-model#status>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Active {
    /// Akahu can authenticate with the institution to retrieve data
    /// and/or initiate payments for this account.
    Active,
    /// Akahu no longer has access to this account. Your
    /// application will still be able to access Akahu's cached copy of data for
    /// this account, but this will no longer be updated by
    /// [refreshes](https://developers.akahu.nz/docs/data-refreshes). Write
    /// actions such as payments or transfers will no longer be available. Once
    /// an account is assigned the INACTIVE status, it will stay this way until
    /// the user re-establishes the connection. When your application observes
    /// an account with a status of INACTIVE, the user should be directed back
    /// to the Akahu OAuth flow or to [<https://my.akahu.nz/connections>] where
    /// they will be prompted to re-establish the connection.
    Inactive,
}

/// This is a less defined part of our API that lets us expose data that may be
/// specific to certain account types or financial institutions. An investment
/// provider, for example, may expose a breakdown of investment results.
///
/// Akahu standardises this metadata as much as possible. However depending on
/// the specific integration and account, some data fields may be unavailable or
/// poorly specified. Treat all fields in the meta object as optional.
///
/// [<https://developers.akahu.nz/docs/the-account-model#meta>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AccountMetadata {
    /// The account holder name as exposed by the provider. In the case of bank
    /// accounts this is the name on the bank statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,

    /// Indicates if the account has other holders that are not listed in the
    /// holder field. This only applies to official open banking connections
    /// where the institution indicates a joint account, but only provides the
    /// authorising party's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_unlisted_holders: Option<bool>,

    /// If the account can be paid but is not a bank account (for example a
    /// KiwiSaver account), this field will have payment details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetails>,

    /// Includes detailed information related to a loan account (if available
    /// from the loan provider).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_details: Option<LoanDetails>,

    /// An investment breakdown. Details are passed straight through from
    /// integrations, making them very inconsistent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<serde_json::Value>,

    /// An investment portfolio. Details are passed through from integrations,
    /// so some are missing various fields. A maximum of 200 funds/instruments
    /// are supported per investment account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portfolio: Option<serde_json::Value>,
}

/// Details for making a payment to an account that is not a bank account.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PaymentDetails {
    /// The recipient's name.
    pub account_holder: String,
    /// The recipient's NZ bank account number.
    pub account_number: BankAccountNumber,
    /// Details required to be in the payment particulars.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub particulars: Option<String>,
    /// Details required to be in the payment code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Details required to be in the payment reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// If there is a minimum amount in order to have the payment accepted, in
    /// dollars.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::arbitrary_precision_option"
    )]
    pub minimum_amount: Option<rust_decimal::Decimal>,
}

/// Detailed information related to a loan account.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct LoanDetails {
    /// The purpose of the loan (E.g. HOME), if we can't determine the purpose,
    /// this will be UNKNOWN.
    pub purpose: String,
    /// The type of loan (E.g. TABLE), if we can't determine the type, this will
    /// be UNKNOWN.
    // TODO: Could be an enum but we do not know all possible classifications.
    #[serde(rename = "type")]
    pub loan_type: String,
    /// Interest rate information for the loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interest: Option<InterestDetails>,
    /// Is the loan currently in an interest only period?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_interest_only: Option<bool>,
    /// When the interest only period expires, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interest_only_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The duration/term of the loan for it to be paid to completion from the
    /// start date of the loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// When the loan matures, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matures_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The loan initial principal amount, this was the original amount
    /// borrowed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::arbitrary_precision_option"
    )]
    pub initial_principal: Option<rust_decimal::Decimal>,
    /// Loan repayment information if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repayment: Option<RepaymentDetails>,
}

/// Interest rate information for a loan.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct InterestDetails {
    /// The rate of interest.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub rate: rust_decimal::Decimal,
    /// The type of interest rate (E.g. FIXED).
    // TODO: Could be an enum but we do not know all possible classifications.
    #[serde(rename = "type")]
    pub interest_type: String,
    /// When this interest rate expires, if available.
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Loan repayment information.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RepaymentDetails {
    /// The frequency of the loan repayment (E.g. MONTHLY).
    pub frequency: String,
    /// The next repayment date, if available.
    pub next_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The next instalment amount.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub next_amount: rust_decimal::Decimal,
}

/// Akahu can refresh different parts of an account's data at different rates.
/// The timestamps in the refreshed object tell you when that account data was
/// last updated.
///
/// When looking at a timestamp in here, you can think "Akahu's view of the
/// account (balance/metadata/transactions) is up to date as of $TIME".
///
/// [<https://developers.akahu.nz/docs/the-account-model#refreshed>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RefreshDetails {
    /// When the balance was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<chrono::DateTime<chrono::Utc>>,

    /// When other account metadata was last updated (any account property apart
    /// from balance).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<chrono::DateTime<chrono::Utc>>,

    /// When we last checked for and processed any new transactions.
    ///
    /// This flag may be missing when an account has first connected, as it
    /// takes a few seconds for new transactions to be processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<chrono::DateTime<chrono::Utc>>,

    /// When we last fetched identity data about the
    /// [party](https://developers.akahu.nz/docs/enduring-identity-verification#party-data)
    /// who has authenticated with the financial institution when connecting
    /// this account.
    ///
    /// This data is updated by Akahu on a fixed 30 day interval, regardless of
    /// your app's [data
    /// refresh](https://developers.akahu.nz/docs/data-refreshes) configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<chrono::DateTime<chrono::Utc>>,
}

/// The account balance.
///
/// [<https://developers.akahu.nz/docs/the-account-model#balance>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct BalanceDetails {
    /// The current account balance.
    ///
    /// A negative balance indicates the amount owed to the account issuer. For
    /// example a checking account in overdraft will have a negative balance,
    /// same as the amount owed on a credit card or the principal remaining on a
    /// loan.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub current: rust_decimal::Decimal,

    /// The balance that is currently available to the account holder.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::arbitrary_precision_option"
    )]
    pub available: Option<rust_decimal::Decimal>,

    /// The credit limit for this account.
    ///
    /// For example a credit card limit or an overdraft limit. This value is
    /// only present when provided directly by the connected financial
    /// institution.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::arbitrary_precision_option"
    )]
    pub limit: Option<rust_decimal::Decimal>,

    /// A boolean indicating whether this account is in overdraft.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overdrawn: Option<bool>,

    /// The [3 letter ISO 4217 currency code](https://www.xe.com/iso4217.php)
    /// that this balance is in (e.g. NZD).
    pub currency: iso_currency::Currency,
}

/// What sort of account this is. Akahu provides specific bank account types,
/// and falls back to more general types for other types of connection.
///
/// [<https://developers.akahu.nz/docs/the-account-model#type>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BankAccountKind {
    /// An everyday spending account.
    Checking,
    /// A savings account.
    ///
    /// NOTE: A savings account is not necessarily a regular bank account. It might
    /// not have transactions associated, or be able to receive payments. Check
    /// the attributes field to see what this account can do.
    Savings,
    /// A credit card.
    #[serde(rename = "CREDITCARD")]
    CreditCard,
    /// A loan account.
    Loan,
    /// A KiwiSaver investment product.
    Kiwisaver,
    /// A general investment product.
    Investment,
    /// A term deposit.
    #[serde(rename = "TERMDEPOSIT")]
    TermDeposit,
    /// An account holding a foreign currency.
    Foreign,
    /// An account with tax authorities.
    Tax,
    /// An account for rewards points, e.g. Fly Buys or True Rewards.
    Rewards,
    /// Available cash for investment or withdrawal from an investment provider.
    Wallet,
}

/// The list of attributes indicates what abilities an account has.
///
/// [<https://developers.akahu.nz/docs/the-account-model#attributes>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Attribute {
    /// Akahu can fetch available transactions from this account.
    Transactions,
    /// This account can receive transfers from accounts belonging to the same
    /// set of credentials.
    TransferTo,
    /// This account can initiate transfers to accounts belonging to the same
    /// set of credentials.
    TransferFrom,
    /// This account can receive payments from another bank account.
    PaymentTo,
    /// This account can initiate payments to another bank account.
    PaymentFrom,
}
