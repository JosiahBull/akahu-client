//! Data refresh CLI example for Akahu client.
//!
//! This example demonstrates how to trigger data refreshes for accounts and connections.
//! Refreshes are asynchronous - they initiate a data update but don't return the updated data.
//!
//! # Authentication
//!
//! Requires two tokens:
//! - User token (AKAHU_USER_TOKEN or --user-token)
//! - App token (AKAHU_APP_TOKEN or --app-token)
//!
//! # Usage Examples
//!
//! Refresh all accounts:
//! ```bash
//! cargo run --example refresh all
//! ```
//!
//! Refresh a specific account:
//! ```bash
//! cargo run --example refresh account --id acc_123...
//! ```
//!
//! Refresh a specific connection:
//! ```bash
//! cargo run --example refresh account --id conn_123...
//! ```

#![allow(
    clippy::all,
    clippy::unwrap_used,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::string_slice,
    clippy::else_if_without_else,
    reason = "Don't care about lints in examples."
)]

use akahu_client::{AkahuClient, UserToken};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

/// Data refresh operations for Akahu accounts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// User access token (can also be set via AKAHU_USER_TOKEN env var)
    #[arg(short = 'u', long, env = "AKAHU_USER_TOKEN")]
    user_token: String,

    /// Application ID token (can also be set via AKAHU_APP_TOKEN env var)
    #[arg(short = 'a', long, env = "AKAHU_APP_TOKEN")]
    app_token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Refresh all accounts
    All {
        /// Show verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Refresh specific account or connection
    Account {
        /// Account ID (acc_...) or Connection ID (conn_...)
        #[arg(short = 'i', long)]
        id: String,

        /// Show verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    // Parse command-line arguments
    let args = Args::parse();

    // Create the Akahu client
    let client = AkahuClient::new(reqwest::Client::new(), args.app_token, None);

    // Convert user token to UserToken type
    let user_token = UserToken::new(args.user_token);

    // Execute the appropriate command
    match args.command {
        Commands::All { verbose } => {
            if verbose {
                eprintln!("Initiating refresh for all accounts...");
            }

            client
                .refresh_all_accounts(&user_token)
                .await
                .context("Failed to initiate refresh for all accounts")?;

            println!("✓ Refresh initiated for all accounts");

            if verbose {
                eprintln!("\nNote: Account data is refreshed and enriched asynchronously.");
                eprintln!("The refresh process may take a few moments to complete.");
            }
        }

        Commands::Account { id, verbose } => {
            if verbose {
                eprintln!("Initiating refresh for {}...", id);
            }

            client
                .refresh_account_or_connection(&user_token, &id)
                .await
                .with_context(|| format!("Failed to initiate refresh for {}", id))?;

            println!("✓ Refresh initiated for {}", id);

            if verbose {
                if id.starts_with("acc_") {
                    eprintln!("\nNote: Refreshing an account will also refresh other accounts");
                    eprintln!("      sharing the same login credentials.");
                } else if id.starts_with("conn_") {
                    eprintln!("\nNote: Refreshing a connection will refresh all accounts");
                    eprintln!("      held at that financial institution.");
                }
                eprintln!("\nAccount data is refreshed and enriched asynchronously.");
                eprintln!("The refresh process may take a few moments to complete.");
            }
        }
    }

    Ok(())
}
