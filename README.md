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
use akahu_client::{AkahuClient, UserToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your app token
    let client = AkahuClient::new(
        reqwest::Client::new(),
        "app_token_...".to_string(),
        None
    );

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
