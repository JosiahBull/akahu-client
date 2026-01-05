//! Connections Endpoints
//!
//! This module contains methods for interacting with financial institution connections.

use crate::ConnectionId;

use super::AkahuClient;
use reqwest::Method;

impl AkahuClient {
    // ==================== Connections Endpoints ====================

    /// Get a list of all connected financial institutions.
    ///
    /// This endpoint provides a list of all financial institutions that users can connect
    /// to your Akahu application.
    ///
    /// **Authentication:** Requires app-scoped authentication (HTTP Basic Auth with app_id_token:app_secret).
    ///
    /// **Note:** App-scoped endpoints are not available for Personal Apps.
    /// You must call `with_app_secret()` on the client before using this endpoint.
    ///
    /// # Returns
    ///
    /// A response containing all available financial institution connections.
    /// Access the connections via the `.items` field.
    ///
    /// [<https://developers.akahu.nz/reference/get_connections>]
    pub async fn get_connections(
        &self,
    ) -> crate::error::AkahuResult<crate::models::ListResponse<crate::models::Connection>> {
        const URI: &str = "connections";

        let headers = self.build_app_headers()?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get a specific financial institution connection by its ID.
    ///
    /// This endpoint retrieves details for an individual financial institution connection.
    ///
    /// **Authentication:** Requires app-scoped authentication (HTTP Basic Auth with app_id_token:app_secret).
    ///
    /// **Note:** App-scoped endpoints are not available for Personal Apps.
    /// You must call `with_app_secret()` on the client before using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `connection_id` - The unique identifier for the connection
    ///
    /// # Returns
    ///
    /// A response containing the connection details.
    /// Access the connection via the `.item` field.
    ///
    /// [<https://developers.akahu.nz/reference/get_connections-id>]
    pub async fn get_connection(
        &self,
        connection_id: &ConnectionId,
    ) -> crate::error::AkahuResult<crate::models::ItemResponse<crate::models::Connection>> {
        let uri = format!("connections/{}", connection_id.as_str());

        let headers = self.build_app_headers()?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }
}
