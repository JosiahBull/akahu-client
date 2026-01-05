//! User profile models for the Akahu API.
//!
//! These models represent the user's profile information that can be accessed
//! via the `/me` endpoint. The visibility of certain fields depends on the
//! permissions granted to your application.

/// Represents the authenticated user's profile information.
///
/// This model contains basic information about the user who authorized your application.
/// The availability of certain fields (such as email) depends on whether your application
/// has been granted the appropriate scopes (e.g., `AKAHU` scope for email access).
///
/// [<https://developers.akahu.nz/reference/get_me>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct User {
    /// The unique identifier for the user in the Akahu system.
    ///
    /// This ID is always prefixed with `user_` so that you can tell it belongs to a user.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    #[serde(rename = "_id")]
    pub id: String,

    /// The timestamp when this user account was created.
    ///
    /// This represents when the user first registered with Akahu.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The user's first name.
    ///
    /// This is the first name the user provided when registering with Akahu.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The user's last name.
    ///
    /// This is the last name the user provided when registering with Akahu.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The user's email address.
    ///
    /// This is the email address the user used to register their Akahu account.
    ///
    /// **Note:** This field is only visible if your application has been granted
    /// the `AKAHU` scope, which provides access to the user's profile information.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The timestamp when the user granted access to your application.
    ///
    /// This represents when the user authorized your app through the OAuth flow.
    ///
    /// [<https://developers.akahu.nz/reference/get_me>]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_granted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Standard response wrapper for a single user item.
///
/// This follows Akahu's standard response format where successful responses
/// have a `success` field set to `true` and the actual data in the `item` field.
///
/// [<https://developers.akahu.nz/docs/response-formatting>]
#[derive(Debug, serde::Deserialize, Clone)]
pub struct UserResponse {
    /// Indicates if the request was successful.
    pub success: bool,

    /// The user data.
    pub item: User,
}
