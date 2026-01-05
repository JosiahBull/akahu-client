//! Connection models for the Akahu API.
//!
//! Connections represent financial institutions that users can connect to.

/// A connection to a financial institution.
///
/// [<https://developers.akahu.nz/reference/get_connections>]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Connection {
    /// Unique identifier for the connection
    #[serde(rename = "_id")]
    pub id: String,

    /// Name of the financial institution
    pub name: String,

    /// URL to the institution's logo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<url::Url>,

    /// Additional metadata about the connection
    #[serde(flatten)]
    pub additional_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
