//! HTTP utility functions for URL building and header conversion

#[cfg(feature = "reqwest")]
use crate::error::{AkahuError, AkahuResult};

/// Build a URL with query parameters
///
/// # Arguments
///
/// * `base` - The base URL
/// * `path` - The path to append to the base URL
/// * `params` - Query parameters as key-value pairs
///
/// # Returns
///
/// Returns the complete URL with encoded query parameters
pub fn build_url(base: &str, path: &str, params: &[(&str, String)]) -> String {
    let mut url = format!("{}/{}", base, path);
    if !params.is_empty() {
        url.push('?');
        let query: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect();
        url.push_str(&query.join("&"));
    }
    url
}

/// Convert a `reqwest::header::HeaderMap` to an `http::HeaderMap`
///
/// This function is only available when the `reqwest` feature is enabled.
///
/// # Arguments
///
/// * `headers` - The reqwest HeaderMap to convert
///
/// # Returns
///
/// Returns a Result containing the converted http::HeaderMap
#[cfg(feature = "reqwest")]
pub fn convert_headers(
    headers: reqwest::header::HeaderMap,
) -> AkahuResult<http::HeaderMap> {
    let mut http_headers = http::HeaderMap::new();
    for (name, value) in headers.iter() {
        let header_name = http::header::HeaderName::from_bytes(name.as_str().as_bytes())
            .map_err(|e| AkahuError::InvalidHeader(e.to_string()))?;
        let header_value = http::header::HeaderValue::from_bytes(value.as_bytes())
            .map_err(|e| AkahuError::InvalidHeader(e.to_string()))?;
        http_headers.insert(header_name, header_value);
    }
    Ok(http_headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_without_params() {
        let url = build_url("https://api.example.com", "users", &[]);
        assert_eq!(url, "https://api.example.com/users");
    }

    #[test]
    fn test_build_url_with_params() {
        let url = build_url(
            "https://api.example.com",
            "users",
            &[("name", "John Doe".to_string()), ("age", "30".to_string())],
        );
        assert_eq!(url, "https://api.example.com/users?name=John%20Doe&age=30");
    }

    #[test]
    fn test_build_url_with_special_chars() {
        let url = build_url(
            "https://api.example.com",
            "search",
            &[("q", "hello&world".to_string())],
        );
        assert_eq!(url, "https://api.example.com/search?q=hello%26world");
    }
}
