//! HTTP client abstraction layer using tower::Service
//!
//! This module provides integration with the `tower::Service` trait, allowing the Akahu client
//! to work with any HTTP client that implements `Service<Request, Response = Response>`.

#[cfg(feature = "reqwest")]
mod reqwest_client;
mod util;

#[cfg(feature = "reqwest")]
pub use reqwest_client::ReqwestClient;
pub use util::build_url;
#[cfg(feature = "reqwest")]
pub use util::convert_headers;

use tower::Service;

/// Extension trait for tower::Service to make it easier to use with immutable references.
///
/// Since `tower::Service::call` requires `&mut self`, but our client methods use `&self`,
/// this trait provides a helper method that clones the service before calling it.
/// This is the standard pattern for using tower services in shared contexts.
pub trait ServiceExt<Request>: Service<Request> + Clone {
    /// Call the service with an immutable reference by cloning it first.
    ///
    /// This is useful when you need to call a service from a context that only has
    /// `&self` access. The service will be cloned (which should be cheap for most
    /// HTTP client implementations) and then called.
    fn call_cloned(&self, request: Request) -> <Self as Service<Request>>::Future {
        let mut svc = self.clone();
        svc.call(request)
    }
}

// Blanket implementation for all types that implement Service + Clone
impl<T, Request> ServiceExt<Request> for T where T: Service<Request> + Clone {}

/// Trait for HTTP services that can be used with AkahuClient.
///
/// This is a convenience trait that bundles all the requirements for an HTTP service.
/// It is automatically implemented for any type that satisfies the bounds.
///
/// Requirements:
/// - Implement `Service<http::Request<Vec<u8>>>`
/// - Return `http::Response<Vec<u8>>` as the response type
/// - Have an error type that is `Send + Sync + 'static` and implements `std::error::Error`
/// - Be `Clone` (cheap cloning is expected, typically via `Arc` internally)
/// - Be `Send + Sync + 'static` for use in async contexts
/// - Have a Future that is `Send`
pub trait HttpService
where
    Self: Service<http::Request<Vec<u8>>, Response = http::Response<Vec<u8>>>
        + Clone
        + Send
        + Sync
        + 'static,
    Self::Error: std::error::Error + Send + Sync + 'static,
    Self::Future: Send,
{
}

// Blanket implementation for all types that satisfy the bounds
impl<T> HttpService for T
where
    T: Service<http::Request<Vec<u8>>, Response = http::Response<Vec<u8>>>
        + Clone
        + Send
        + Sync
        + 'static,
    T::Error: std::error::Error + Send + Sync + 'static,
    T::Future: Send,
{
}
