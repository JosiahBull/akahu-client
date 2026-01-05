//! Connection models for the Akahu API.
//!
//! Connections represent financial institutions that users can connect to.

use serde::{Deserialize, Serialize};

use crate::ConnectionId;

/// A connection to a financial institution.
///
/// [<https://developers.akahu.nz/reference/get_connections>]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Connection {
    /// Unique identifier for the connection
    #[serde(rename = "_id")]
    pub id: ConnectionId,

    /// Name of the financial institution
    pub name: String,

    /// URL to the institution's logo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<url::Url>,

    /// Additional metadata about the connection
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}
