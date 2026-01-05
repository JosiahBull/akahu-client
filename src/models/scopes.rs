/// Scopes that can be requested for an OAuth flow.
///
/// As part of Akahu's dedication to privacy and the security of personal data, we use a scope
/// system to provide only the data an app requires to function (i.e. The Principle of Least
/// Privilege).
///
/// As an app developer, you will need to specify and give reasons for which scopes your app
/// requires when you sign up to create an app. These scopes are enforced at the data access
/// level, so your app will never be able to access data for which it doesn't have permission,
/// nor ask users to grant access to data the app is not allowed to view.
///
/// [<https://developers.akahu.nz/docs/scopes>]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Scope {
    // Enduring consent scopes
    /// **(Required for enduring consent)**
    /// Gives your app ongoing permission to access the user's accounts.
    /// Supply this scope in an OAuth request to begin an enduring consent flow.
    EnduringConsent,
    /// **(Optional)**
    /// Gives your app access to the user's profile information held by Akahu,
    /// such as the email address they used to register their Akahu account.
    Akahu,
    /// **(Optional)**
    /// Gives access to the user's connected accounts. You will only be able to view the
    /// accounts shared with you by the user. The account data visible to your app is also
    /// limited, depending on whether your app needs access to balances, metadata, or
    /// account holder details.
    Accounts,
    /// **(Optional)**
    /// Gives access to the user's transactions. You will only be able to view transactions
    /// from accounts shared with you by the user. Further restrictions may be applied
    /// including limiting the date window viewable for your app or limiting the categories
    /// of transactions visible to your app.
    ///
    /// *Note: This scope is available for both enduring and one-off consent.*
    Transactions,
    /// **(Optional)**
    /// Gives access to our transfer API, allowing your app to move money between a user's
    /// accounts you have been granted access to.
    Transfers,
    /// **(Optional)**
    /// Gives access to our payments API, allowing your app to send money to any account
    /// number from accounts you have been granted access to.
    Payments,
    /// **(Optional)**
    /// Gives access to the user's official name as retrieved from connected accounts.
    IdentityNames,
    /// **(Optional)**
    /// Gives access to the user's date of birth as retrieved from connected accounts.
    IdentityDobs,
    /// **(Optional)**
    /// Gives access to the user's email addresses as retrieved from connected accounts.
    IdentityEmails,
    /// **(Optional)**
    /// Gives access to the user's phone numbers as retrieved from connected accounts.
    IdentityPhones,
    /// **(Optional)**
    /// Gives access to the user's tax numbers (IRD numbers) as retrieved from connected accounts.
    IdentityTaxNumbers,

    // One-off consent scopes
    /// **(Required for one-off consent)**
    /// Gives your app permission to access a user's data at the time you request it.
    /// Supply this scope in an OAuth request to begin a one-off connection flow.
    Oneoff,
    /// **(Optional)**
    /// Gives access to the user's account holder information, as supplied by the
    /// connected institution.
    Holder,
    /// **(Optional)**
    /// Gives access to the user's residential and postal address, as supplied by the
    /// connected institution.
    Address,
    /// **(Optional)**
    /// Gives access to the user's account details, including the holder name, account
    /// number, and branch details, as supplied by the connected institution.
    Account,
    /// **(Optional)**
    /// Gives your app permission to access a user's bank statements.
    Statements,
    /// **(Optional)**
    /// Gives your app permission to access a user's bank transactions in PDF format.
    PdfExports,
}
