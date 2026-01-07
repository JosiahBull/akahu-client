# akahu-client

A non-offical Rust client library for the [Akahu API](https://www.akahu.nz/),
providing access to financial data aggregation services in New Zealand.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
akahu-client = "0.1.0"
```

## Quick Start

```rust
use akahu_client::{AkahuClient, ReqwestClient, UserToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client (using reqwest - enabled by default)
    let http_client = ReqwestClient::new(reqwest::Client::new());

    // Create a client with your app token
    let client = AkahuClient::new(
        http_client,
        "app_token_...".to_string(),
        None
    );

    // Or use the convenience method
    let client = AkahuClient::with_reqwest("app_token_...");

    // Create a user token from OAuth flow
    let user_token = UserToken::new("user_token_...".to_string());

    // Fetch accounts
    let accounts = client.get_accounts(&user_token).await?;

    for account in accounts.items {
        println!("{}: {} - {:.2}",
            account.name,
            account.kind,
            account.balance.current
        );
    }

    Ok(())
}
```

## Features

- **Tower Service Integration**: Uses the standard `tower::Service` trait for HTTP client abstraction
- **Default reqwest Support**: Includes built-in support for reqwest (enabled by default)
- **Type-safe API**: Strongly-typed models for all API responses
- **Async/await**: Built on tokio for efficient async operations
- **Tower Middleware**: Compatible with the entire tower middleware ecosystem

### Using a Custom HTTP Client

You can use any HTTP library by implementing `tower::Service`:

```rust
use tower::Service;
use std::task::{Context, Poll};
use http::{Request, Response};

#[derive(Clone)]
struct MyHttpClient;

impl Service<Request<Vec<u8>>> for MyHttpClient {
    type Response = Response<Vec<u8>>;
    type Error = MyError;
    type Future = /* your future type */;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Vec<u8>>) -> Self::Future {
        // Your HTTP client implementation
    }
}

let client = AkahuClient::new(MyHttpClient, "app_token_...", None);
```

The library uses the standard `tower::Service` trait, giving you full access to tower's middleware ecosystem for features like retries, rate limiting, timeouts, and more.

### Feature Flags

- `reqwest` (default): Enables the reqwest HTTP client implementation

To disable the reqwest feature and use your own HTTP client:

```toml
[dependencies]
akahu-client = { version = "0.1.0", default-features = false }
tower = "0.5"  # Required for implementing Service
http = "1.1"   # Required for HTTP types
```

When the `reqwest` feature is disabled, you must explicitly specify the HTTP client type:

```rust
// With reqwest enabled (default):
let client = AkahuClient::with_reqwest("app_token_...");

// Without reqwest - must specify type:
let client: AkahuClient<MyHttpClient> = AkahuClient::new(MyHttpClient::new(), "app_token_...", None);
```

## Validation

Note that I only use this in a very limited context, mostly for accounts/transactions. If you
need APIs that I haven't tested please validate them yourself and open issues/PRs for any problems
that you encounter.

Some APIs I have not bothered to port over - but I would welcome PRs or requests.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [Akahu API Documentation](https://developers.akahu.nz/)
- [Akahu Website](https://www.akahu.nz/)
