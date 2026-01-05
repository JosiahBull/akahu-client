use async_trait::async_trait;
use reqwest::{
    Method, StatusCode,
    header::{HeaderMap, HeaderValue},
};
use std::{collections::HashMap, error::Error as StdError};

pub const DEFAULT_BASE_URL: &'static str = "https://api.akahu.io/v1";

pub struct AkahuClient {
    // TODO: generic type to allow other http clients, e.g. reqwest, reqwest_middleware, etc.
    client: reqwest::Client,
    app_id_token: String,
    base_url: String,
}

impl AkahuClient {
    // TODO: use bon to create builder interfaces over everything.
    pub fn new(client: reqwest::Client, app_id_token: String, base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        Self {
            client,
            app_id_token,
            base_url,
        }
    }
}

impl AkahuClient {
    /// Get a list of all accounts that the user has connected to your application.
    ///
    /// [<https://developers.akahu.nz/reference/get_accounts>]
    pub async fn get_accounts(&self) -> Vec<crate::models::Account> {
        const URI: &str = "accounts";

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Akahu-Id",
            HeaderValue::from_str(&self.app_id_token).unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let req = self
            .client
            .request(Method::GET, format!("{}/{}", self.base_url, URI))
            .headers(headers)
            .build()
            .unwrap();

        let res = self.client.execute(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let res: Vec<crate::models::Account> = res.json().await.unwrap();

        res
    }

    /// Get a list of the user's transactions within the start and end time range.
    ///
    /// This endpoint returns settled transactions for all accounts that the user has connected to your application. See GET /transactions/pending to also query pending transactions.
    ///
    ///     ️ ℹ️ Time range defaults to the entire range accessible to your app.
    ///
    /// Some important things to know when querying transactions:
    ///
    ///     Transactions will look different depending on your app's permissions.
    ///     All transactions timestamps are in UTC.
    ///     The start query parameter is exclusive.
    ///     The end query parameter is inclusive.
    ///     All Akahu timestamps use millisecond resolution (i.e. 2025-01-01T11:59:59.999Z is the instant before 2025-01-01T12:00:00.000Z).
    ///
    /// For more details see:
    pub async fn get_transactions(
        &self,
        token: &str,
        range: std::ops::RangeInclusive<chrono::DateTime<chrono::Utc>>,
    ) -> Vec<crate::models::Transaction> {
        const URI: &str = "transactions";

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Akahu-Id",
            HeaderValue::from_str(&self.app_id_token).unwrap(),
        );
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let mut query_params = HashMap::new();
        query_params.insert(
            "start",
            range
                .start()
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        );
        query_params.insert(
            "end",
            range
                .end()
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        );
        query_params.insert("cursor", "".to_string());

        let url = format!("{}/{}", self.base_url, URI);
        let url = reqwest::Url::parse_with_params(&url, &query_params).unwrap();

        println!("url: {}", url);

        let req = self
            .client
            .request(Method::GET, url)
            .headers(headers)
            .build()
            .unwrap();

        let res = self.client.execute(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let data: serde_json::Value = res.json().await.unwrap();
        panic!("data: {:#?}", data);
    }
}

#[tokio::test]
async fn owo() {
    let client = AkahuClient::new(
        reqwest::Client::new(),
        "app_token_cmjz7wks7000102jvhk5jc2n5".to_string(),
        None,
    );

    // Get transactions from the last month
    let now = chrono::Utc::now();
    let one_month_ago = now - chrono::Duration::days(30);
    let transactions = client.get_transactions(one_month_ago..=now).await;
    println!("Transactions: {:#?}", transactions);

    panic!("done");
}
