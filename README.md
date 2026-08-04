# akahu-client

A non-offical Rust client library for the [Akahu API](https://www.akahu.nz/),
providing access to financial data aggregation services in New Zealand.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
akahu-client = "0.3.0"
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

## Unknown values from Akahu

Every enum here that mirrors a set of strings Akahu chooses - a transaction `type`, an account
`type`/`status`/`attribute`, an identity `type` - has an `Unknown` catch-all variant and is
`#[non_exhaustive]`.

This is not cosmetic. Akahu can add to those vocabularies whenever it likes, a page of results is
deserialised as one value, and so a single value this crate had never heard of used to fail all 100
transactions it arrived with. If your sync only advances its cursor on success, it then refetches the
same window and fails on it again, forever, until this crate is republished. `Unknown` turns that
into one uninteresting field.

`Unknown` doesn't carry the string it stood in for - you still have the response it came from. See
the crate docs for the full reasoning.

## Response body ceiling

A timeout bounds how *long* a response takes, not how many bytes it is, and the body has to be
buffered whole before it can be deserialised. Responses are therefore capped at
`DEFAULT_MAX_RESPONSE_BYTES` (8 MiB, about two orders of magnitude above Akahu's largest documented
page) and rejected with `AkahuError::ResponseTooLarge`. Raise it if you need to:

```rust
use akahu_client::AkahuClient;

let client = AkahuClient::new(reqwest::Client::new(), "app_token_...".to_string(), None)
    .with_max_response_bytes(64 * 1024 * 1024);
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
