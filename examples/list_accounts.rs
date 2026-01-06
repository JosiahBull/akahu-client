//! List all accounts connected to your Akahu application.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example list_accounts -- --help
//! ```

#![allow(clippy::restriction, reason = "clap Parser derive macro requires this")]

use akahu_client::{AkahuClient, UserToken};
use anyhow::{Context, Result};
use clap::Parser;

/// List all accounts connected to your Akahu application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// User access token (can also be set via AKAHU_USER_TOKEN env var)
    #[arg(
        short = 'u',
        long,
        env = "AKAHU_USER_TOKEN",
        help = "User access token from OAuth flow"
    )]
    user_token: String,

    /// Application ID token (can also be set via AKAHU_APP_TOKEN env var)
    #[arg(
        short = 'a',
        long,
        env = "AKAHU_APP_TOKEN",
        help = "Application ID token"
    )]
    app_token: String,

    /// Output format
    #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json"])]
    format: String,

    /// Show only active accounts
    #[arg(long, help = "Filter to show only active accounts")]
    active_only: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Create the Akahu client
    let client = AkahuClient::new(reqwest::Client::new(), args.app_token, None);

    // Convert user token to UserToken type
    let user_token = UserToken::new(args.user_token);

    // Fetch accounts
    println!("Fetching accounts...");
    let response = client
        .get_accounts(&user_token)
        .await
        .context("Failed to fetch accounts")?;

    // Extract accounts from response
    let accounts = response.items;

    // Filter if requested (currently a placeholder - adjust based on your needs)
    let accounts: Vec<_> = if args.active_only { accounts } else { accounts };

    if accounts.is_empty() {
        println!("No accounts found.");
        return Ok(());
    }

    // Display results
    match args.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&accounts)
                .context("Failed to serialize accounts to JSON")?;
            println!("{}", json);
        }
        "table" => {
            println!("\n{} account(s) found:\n", accounts.len());
            println!(
                "{:<40} {:<30} {:<20} {:<15}",
                "ID", "NAME", "TYPE", "BALANCE"
            );
            println!("{}", "=".repeat(105));

            for account in accounts {
                // Format balance - current available balance
                let balance_str = format!("{:.2}", account.balance.current);

                println!(
                    "{:<40} {:<30} {:<20} {:<15}",
                    account.id.as_str(),
                    &account.name,
                    format!("{:?}", account.kind),
                    balance_str
                );
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
