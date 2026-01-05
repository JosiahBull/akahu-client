//! Category endpoint implementations.
//!
//! This module contains methods for retrieving NZFCC (New Zealand Financial Category Codes) categories.

use super::AkahuClient;
use reqwest::Method;

impl AkahuClient {
    /// Get a list of all NZFCC (New Zealand Financial Category Codes) categories.
    ///
    /// This endpoint provides general functionality not tied to individual users.
    ///
    /// **Authentication:** Requires app-scoped authentication (HTTP Basic Auth with app_id_token:app_secret).
    ///
    /// **Note:** App-scoped endpoints are not available for Personal Apps.
    /// You must call `with_app_secret()` on the client before using this endpoint.
    ///
    /// # Returns
    ///
    /// A vector of all available NZFCC categories.
    ///
    /// [<https://developers.akahu.nz/reference/get_categories>]
    pub async fn get_categories(&self) -> crate::error::AkahuResult<Vec<crate::models::Category>> {
        const URI: &str = "categories";

        let headers = self.build_app_headers()?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }

    /// Get a specific NZFCC category by its ID.
    ///
    /// This endpoint provides general functionality not tied to individual users.
    ///
    /// **Authentication:** Requires app-scoped authentication (HTTP Basic Auth with app_id_token:app_secret).
    ///
    /// **Note:** App-scoped endpoints are not available for Personal Apps.
    /// You must call `with_app_secret()` on the client before using this endpoint.
    ///
    /// # Arguments
    ///
    /// * `category_id` - The unique identifier for the category
    ///
    /// # Returns
    ///
    /// The category details.
    ///
    /// [<https://developers.akahu.nz/reference/get_categories-id>]
    pub async fn get_category(
        &self,
        category_id: &str,
    ) -> crate::error::AkahuResult<crate::models::Category> {
        let uri = format!("categories/{}", category_id);

        let headers = self.build_app_headers()?;

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, uri))
            .headers(headers)
            .build()?;

        self.execute_request(req).await
    }
}
