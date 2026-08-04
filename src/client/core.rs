//! Core helper methods for the Akahu client.

use crate::UserToken;

use super::AkahuClient;
use reqwest::{
    StatusCode,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};

/// Custom HTTP header name for Akahu application ID
const AKAHU_ID_HEADER: &str = "X-Akahu-Id";

/// The status line's own description of a failure, for when the response body can't supply a
/// better one.
fn canonical_reason(status: StatusCode) -> String {
    status
        .canonical_reason()
        .unwrap_or("Unknown error")
        .to_string()
}

impl AkahuClient {
    /// Execute a request and handle the response, converting HTTP errors to AkahuError
    pub(super) async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::Request,
    ) -> crate::error::AkahuResult<T> {
        let res = self.client.execute(req).await?;

        if res.status().is_success() {
            let body = self.read_body_capped(res).await?;
            // Deserialize straight from the bytes. JSON is UTF-8 by definition, so there is
            // nothing to gain from converting the whole body to a `String` first — and on a
            // page of a hundred transactions that copy is the difference between one buffer
            // and three live at once.
            serde_json::from_slice(&body).map_err(|error| {
                crate::error::AkahuError::JsonDeserialization {
                    error,
                    response_body: Some(crate::error::ResponseBody::from_bytes(&body)),
                }
            })
        } else {
            self.handle_error_response(res).await
        }
    }

    /// Read a response body into memory, refusing to buffer more than
    /// [`DEFAULT_MAX_RESPONSE_BYTES`](super::DEFAULT_MAX_RESPONSE_BYTES) (or whatever
    /// [`AkahuClient::with_max_response_bytes`] was given).
    ///
    /// Two checks, because either alone is bypassable. When the far end declares a
    /// `Content-Length` over the ceiling the read costs nothing but the headers we already
    /// have; but that header is absent on a chunked response and is only ever a claim, so the
    /// body is then accumulated chunk by chunk against a running total and abandoned — the
    /// connection dropped mid-stream rather than drained — the moment it crosses the cap. At
    /// most one chunk beyond the ceiling is ever held.
    ///
    /// [`reqwest::Response::chunk`] rather than `bytes_stream()` on purpose: the latter lives
    /// behind reqwest's `stream` feature, which this crate does not enable (and which would
    /// pull in a `StreamExt` to use). The bound is identical either way.
    async fn read_body_capped(
        &self,
        mut res: reqwest::Response,
    ) -> crate::error::AkahuResult<Vec<u8>> {
        let cap = self.max_response_bytes;
        let declared = res.content_length();

        if declared.is_some_and(|declared| declared > cap) {
            return Err(crate::error::AkahuError::ResponseTooLarge {
                max_bytes: cap,
                declared_bytes: declared,
            });
        }

        let mut body = Vec::new();
        let mut total: u64 = 0;
        while let Some(chunk) = res.chunk().await? {
            total = total.saturating_add(u64::try_from(chunk.len()).unwrap_or(u64::MAX));
            if total > cap {
                return Err(crate::error::AkahuError::ResponseTooLarge {
                    max_bytes: cap,
                    declared_bytes: declared,
                });
            }
            body.extend_from_slice(&chunk);
        }

        Ok(body)
    }

    /// Parse error response and map to appropriate AkahuError variant
    pub(super) async fn handle_error_response<T>(
        &self,
        res: reqwest::Response,
    ) -> crate::error::AkahuResult<T> {
        let status = res.status();

        // An error body is exactly as unbounded as a successful one, so it goes through the
        // same ceiling. Anything that stops us reading a message out of it — an oversized
        // body included — falls back to the status line's own description, rather than
        // reporting a complaint about the body in place of the 401 that actually happened.
        let message = match self.read_body_capped(res).await {
            Ok(body) => serde_json::from_slice::<crate::models::ErrorResponse>(&body).map_or_else(
                |_| canonical_reason(status),
                |error_body| error_body.message,
            ),
            Err(_) => canonical_reason(status),
        };

        Err(match status {
            StatusCode::BAD_REQUEST => crate::error::AkahuError::BadRequest {
                message,
                status: StatusCode::BAD_REQUEST.as_u16(),
            },
            StatusCode::UNAUTHORIZED => crate::error::AkahuError::Unauthorized { message },
            StatusCode::FORBIDDEN => crate::error::AkahuError::Forbidden { message },
            StatusCode::NOT_FOUND => crate::error::AkahuError::NotFound { message },
            StatusCode::TOO_MANY_REQUESTS => crate::error::AkahuError::RateLimited { message },
            StatusCode::INTERNAL_SERVER_ERROR => {
                crate::error::AkahuError::InternalServerError { message }
            }
            _ => crate::error::AkahuError::ApiError {
                status: status.as_u16(),
                message,
            },
        })
    }

    /// Build standard headers for user-scoped requests
    pub(super) fn build_user_headers(
        &self,
        user_token: &UserToken,
    ) -> crate::error::AkahuResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(AKAHU_ID_HEADER, HeaderValue::from_str(&self.app_id_token)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", user_token.as_str()))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "tests need to unwrap to verify correctness"
)]
mod tests {
    use super::*;
    use crate::{AkahuError, DEFAULT_MAX_RESPONSE_BYTES};
    use std::fmt::Write as _;
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
    use tokio::net::TcpListener;

    /// The ceiling is worth exercising through the real read path rather than a
    /// re-implementation of it — the whole reason a caller can't fix this themselves is that
    /// this crate owns the body read. So: serve one canned response on a loopback port and
    /// return the base URL to point a client at. The crate has no other HTTP test
    /// infrastructure, and a raw listener needs none.
    async fn serve_once(response: Vec<u8>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let _server = tokio::spawn(serve(listener, response));
        format!("http://{addr}")
    }

    /// Best-effort by design: the client hangs up the moment it decides a body is too large,
    /// so a write that fails part-way through is the expected outcome of the oversized cases
    /// rather than a test failure. Hence the `Result` nobody looks at.
    async fn serve(listener: TcpListener, response: Vec<u8>) -> std::io::Result<()> {
        let (mut socket, _peer) = listener.accept().await?;
        // Drain what the client sent before answering it. A partial read is fine — the tests
        // don't care what the request says, only that it isn't left unread in the socket.
        let mut head = [0_u8; 1024];
        let _head_len = socket.read(&mut head).await?;
        socket.write_all(&response).await?;
        socket.shutdown().await
    }

    /// A response that declares its length, so the ceiling can be checked before the body is
    /// read at all.
    fn response(status_line: &str, body: &str) -> Vec<u8> {
        let mut out = String::new();
        write!(
            out,
            "HTTP/1.1 {status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .expect("writing to a String cannot fail");
        out.into_bytes()
    }

    /// A chunked response declares no `Content-Length`, so the only way to bound it is to
    /// count the bytes as they arrive.
    fn chunked_response(chunk: &str, chunks: usize) -> Vec<u8> {
        let mut out = String::from(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n",
        );
        for _ in 0..chunks {
            write!(out, "{:x}\r\n{chunk}\r\n", chunk.len())
                .expect("writing to a String cannot fail");
        }
        out.push_str("0\r\n\r\n");
        out.into_bytes()
    }

    fn client(base_url: String, max_response_bytes: u64) -> AkahuClient {
        AkahuClient::new(reqwest::Client::new(), "app_token_test", Some(base_url))
            .with_max_response_bytes(max_response_bytes)
    }

    /// A body inside the ceiling is read and deserialised exactly as it always was.
    #[tokio::test]
    async fn a_normal_response_is_read_and_deserialised() {
        let body =
            r#"{"success":true,"item":{"_id":"user_1","created_at":"2026-01-06T10:00:00.000Z"}}"#;
        let base = serve_once(response("200 OK", body)).await;

        let user = client(base, DEFAULT_MAX_RESPONSE_BYTES)
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap();

        assert_eq!(user.id.as_str(), "user_1");
    }

    /// A declared length over the ceiling is refused before a byte of the body is read, which
    /// is the whole point of looking at `Content-Length` first.
    #[tokio::test]
    async fn an_oversized_declared_body_is_refused() {
        let body = "x".repeat(4096);
        let base = serve_once(response("200 OK", &body)).await;

        let error = client(base, 1024)
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap_err();

        let rendered = error.to_string();
        let AkahuError::ResponseTooLarge {
            max_bytes,
            declared_bytes,
        } = error
        else {
            panic!("expected the body ceiling to reject this, got: {rendered}")
        };
        assert_eq!(max_bytes, 1024);
        assert_eq!(declared_bytes, Some(4096));
    }

    /// The case a `Content-Length` check alone would miss: a chunked response declares no
    /// length, so the ceiling has to be enforced while the body is still arriving.
    #[tokio::test]
    async fn an_oversized_chunked_body_is_refused_mid_read() {
        let base = serve_once(chunked_response(&"x".repeat(512), 8)).await;

        let error = client(base, 1024)
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap_err();

        let rendered = error.to_string();
        let AkahuError::ResponseTooLarge {
            max_bytes,
            declared_bytes,
        } = error
        else {
            panic!("expected the body ceiling to reject this, got: {rendered}")
        };
        assert_eq!(max_bytes, 1024);
        assert_eq!(
            declared_bytes, None,
            "a chunked response declares nothing to report"
        );
    }

    /// A response just inside the ceiling still has to go through, or the check is off by one
    /// somewhere.
    #[tokio::test]
    async fn a_body_exactly_at_the_ceiling_is_accepted() {
        let body =
            r#"{"success":true,"item":{"_id":"user_1","created_at":"2026-01-06T10:00:00.000Z"}}"#;
        let base = serve_once(response("200 OK", body)).await;

        let user = client(base, u64::try_from(body.len()).unwrap())
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap();

        assert_eq!(user.id.as_str(), "user_1");
    }

    /// An error body is exactly as unbounded as a successful one, so it goes through the same
    /// ceiling — but the caller still needs to hear about the 401 that happened, not about the
    /// size of the page describing it.
    #[tokio::test]
    async fn an_oversized_error_body_still_reports_the_status() {
        let body = "x".repeat(4096);
        let base = serve_once(response("401 Unauthorized", &body)).await;

        let error = client(base, 1024)
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap_err();

        let rendered = error.to_string();
        let AkahuError::Unauthorized { message } = error else {
            panic!("expected the 401 to survive an unreadable body, got: {rendered}")
        };
        assert_eq!(message, "Unauthorized");
    }

    /// End to end: a body that parses as JSON but not as the expected type keeps the body for
    /// a debugger and keeps it out of the error's text.
    #[tokio::test]
    async fn a_deserialisation_failure_keeps_the_body_out_of_its_message() {
        let body = r#"{"success":true,"item":{"description":"FLAT WHITE THE ROASTERY"}}"#;
        let base = serve_once(response("200 OK", body)).await;

        let error = client(base, DEFAULT_MAX_RESPONSE_BYTES)
            .get_me(&UserToken::new("user_token_test"))
            .await
            .unwrap_err();

        let rendered = error.to_string();
        assert!(
            !rendered.contains("ROASTERY"),
            "response body leaked into the error message: {rendered}"
        );
        let AkahuError::JsonDeserialization { response_body, .. } = error else {
            panic!("expected a deserialisation error, got: {rendered}")
        };
        assert_eq!(
            response_body
                .expect("the body should be kept for debugging")
                .as_str(),
            body,
            "the body is still available to a caller that asks for it"
        );
    }
}
