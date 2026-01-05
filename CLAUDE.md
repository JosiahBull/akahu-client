# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust client library for the Akahu API, a New Zealand financial data aggregation service. The library provides type-safe access to Akahu's banking and financial data APIs.

## Common Commands

### Build
```bash
cargo build
```

### Run tests
```bash
cargo test
```

### Run a specific test
```bash
cargo test <test_name>
```

### Check code without building
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Run lints
```bash
cargo clippy
```

## Architecture

### Project Structure

The codebase is organized as follows:

- `src/lib.rs` - Library entry point (currently minimal, just module declarations)
- `src/client.rs` - Main `AkahuClient` implementation with API methods
- `src/error.rs` - Error types (currently minimal stub)
- `src/models/` - Data models matching Akahu's API schemas
  - `account.rs` - Account-related types
  - `transaction.rs` - Transaction-related types
  - `webhook.rs` - Webhook types (minimal)
  - `scopes.rs` - OAuth scope definitions

### Client Design

The `AkahuClient` is the primary interface:
- Uses `reqwest::Client` for HTTP requests
- Requires an `app_id_token` for authentication
- Supports configurable `base_url` (defaults to `https://api.akahu.io/v1`)
- Currently implements:
  - `get_accounts()` - Fetch all connected accounts
  - `get_transactions()` - Fetch transactions in a time range (work in progress)

### Authentication Pattern

All API requests follow this pattern:
- Include `X-Akahu-Id` header with the app token
- For user-scoped endpoints, include `Authorization: Bearer <user_token>` header
- Set `Accept: application/json` header

### Data Models

The models are comprehensive Rust representations of Akahu's API:

1. **Account Model** (`models/account.rs`):
   - Rich type-safe representation with enums for status, account types, and attributes
   - Nested structures for balance, refresh times, and metadata
   - Handles optional fields extensively (different integrations provide different data)
   - Uses `rust_decimal::Decimal` for precise financial amounts
   - Uses `iso_currency::Currency` for currency codes
   - Uses `chrono::DateTime<Utc>` for timestamps

2. **Transaction Model** (`models/transaction.rs`):
   - Includes base transaction data and optional enrichment (merchant, categories)
   - Uses NZ FCC codes via the `nzfcc` crate for categorization
   - Supports metadata like payment fields, currency conversions, and card details
   - Uses `rust_decimal::Decimal` for amounts

3. **Scopes** (`models/scopes.rs`):
   - Enum of all OAuth scopes for both enduring and one-off consent flows
   - Well-documented with Akahu's permission model

### Development Status

This is a work-in-progress library:
- TODO comments indicate planned improvements (e.g., builder pattern with `bon`, generic HTTP client support)
- Error handling is currently using `.unwrap()` and `assert!` - needs proper error propagation
- Test code contains panic statements and debug output - not production ready
- The `get_transactions()` endpoint has debug code that needs completion

### Dependencies

Key dependencies and their purposes:
- `reqwest` - HTTP client with JSON support
- `serde` / `serde_json` - Serialization
- `rust_decimal` - Precise decimal arithmetic for financial values
- `chrono` - Date/time handling
- `iso_currency` - Currency code validation
- `nzfcc` - New Zealand Financial Category Codes
- `url` - URL parsing
- `bon` - Builder pattern macros (planned usage)
- `async-trait` - Async trait support
- `tokio` - Async runtime

### Important Patterns

1. **Serde Attributes**: Models heavily use `#[serde(rename = "_id")]` for Akahu's underscore-prefixed ID fields
2. **Optional Fields**: Most nested data uses `#[serde(default, skip_serializing_if = "Option::is_none")]`
3. **Decimal Serialization**: Financial amounts use `#[serde(with = "rust_decimal::serde::arbitrary_precision")]`
4. **Enum Naming**: API enums use `#[serde(rename_all = "UPPERCASE")]` or `#[serde(rename_all = "SCREAMING_SNAKE_CASE")]`

## Key Considerations

- The Akahu API uses millisecond-precision timestamps (e.g., `2025-01-01T11:59:59.999Z`)
- Transaction queries use exclusive start times and inclusive end times
- All timestamps are in UTC
- Account data availability depends on integration type and app permissions
- The API requires different authentication for app-level vs user-level endpoints
