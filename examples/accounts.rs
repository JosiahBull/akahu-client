//! Account management CLI example for Akahu client.
//!
//! This example demonstrates how to:
//! - List all accounts with filtering
//! - Get details for a specific account
//! - Revoke access to accounts
//!
//! # Authentication
//!
//! Requires two tokens:
//! - User token (AKAHU_USER_TOKEN or --user-token)
//! - App token (AKAHU_APP_TOKEN or --app-token)
//!
//! # Usage Examples
//!
//! List all active accounts:
//! ```bash
//! cargo run --example accounts list --active-only
//! ```
//!
//! List accounts filtered by type:
//! ```bash
//! cargo run --example accounts list --account-type CHECKING
//! ```
//!
//! Get specific account as JSON:
//! ```bash
//! cargo run --example accounts get --account-id acc_123... --format json
//! ```
//!
//! Export accounts to CSV:
//! ```bash
//! cargo run --example accounts list --format csv > accounts.csv
//! ```
//!
//! Revoke access to an account:
//! ```bash
//! cargo run --example accounts revoke --account-id acc_123...
//! ```

#![allow(deprecated)]

use akahu_client::{AccountId, AkahuClient, UserToken};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

/// Account management operations for Akahu
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
    /// List all accounts
    List {
        /// Output format
        #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
        format: String,

        /// Show only active accounts
        #[arg(long)]
        active_only: bool,

        /// Filter by account type (e.g., CHECKING, SAVINGS, CREDITCARD)
        #[arg(long)]
        account_type: Option<String>,
    },

    /// Get a specific account
    Get {
        /// Account ID (e.g., acc_123...)
        #[arg(short = 'i', long)]
        account_id: String,

        /// Output format
        #[arg(short = 'f', long, default_value = "json", value_parser = ["json", "table"])]
        format: String,
    },

    /// Revoke access to an account
    Revoke {
        /// Account ID to revoke
        #[arg(short = 'i', long)]
        account_id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
}

mod output {
    use akahu_client::Account;
    use anyhow::{Context, Result};
    use serde::Serialize;

    pub fn format_json<T: Serialize>(items: &T) -> Result<String> {
        serde_json::to_string_pretty(items).context("Failed to serialize to JSON")
    }

    pub fn format_accounts_table(accounts: &[Account]) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{} account(s) found:\n\n", accounts.len()));
        output.push_str(&format!(
            "{:<40} {:<30} {:<20} {:<15}\n",
            "ID", "NAME", "TYPE", "BALANCE"
        ));
        output.push_str(&format!("{}\n", "=".repeat(105)));

        for account in accounts {
            let balance_str = format!("{:.2}", account.balance.current);

            output.push_str(&format!(
                "{:<40} {:<30} {:<20} {:<15}\n",
                account.id.as_str(),
                truncate(&account.name, 30),
                format!("{:?}", account.kind),
                balance_str
            ));
        }

        output
    }

    pub fn format_accounts_csv(accounts: &[Account]) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("ID,Name,Type,Status,Balance,Available,Limit\n");

        // CSV rows
        for account in accounts {
            output.push_str(&format!(
                "{},{},{},{:?},{},{},{}\n",
                account.id.as_str(),
                escape_csv(&account.name),
                format!("{:?}", account.kind),
                account.status,
                account.balance.current,
                account
                    .balance
                    .available
                    .map(|b: rust_decimal::Decimal| b.to_string())
                    .unwrap_or_default(),
                account
                    .balance
                    .limit
                    .map(|l: rust_decimal::Decimal| l.to_string())
                    .unwrap_or_default()
            ));
        }

        output
    }

    pub fn format_single_account_table(account: &Account) -> String {
        let mut output = String::new();

        output.push_str("\nAccount Details:\n\n");
        output.push_str(&format!("ID:                {}\n", account.id.as_str()));
        output.push_str(&format!("Name:              {}\n", account.name));
        output.push_str(&format!("Type:              {:?}\n", account.kind));
        output.push_str(&format!("Status:            {:?}\n", account.status));
        output.push_str(&format!(
            "Current Balance:   {:.2}\n",
            account.balance.current
        ));

        if let Some(available) = account.balance.available {
            output.push_str(&format!("Available:         {:.2}\n", available));
        }

        if let Some(limit) = account.balance.limit {
            output.push_str(&format!("Limit:             {:.2}\n", limit));
        }

        if let Some(formatted) = &account.formatted_acount {
            output.push_str(&format!("Account Number:    {}\n", formatted));
        }

        if let Some(balance_refreshed) = account.refreshed.balance {
            output.push_str(&format!(
                "Last Refreshed:    {}\n",
                balance_refreshed.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        output
    }

    fn truncate(s: &str, max_len: usize) -> String {
        if s.len() > max_len {
            format!("{}...", &s[..max_len - 3])
        } else {
            s.to_string()
        }
    }

    fn escape_csv(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }
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
        Commands::List {
            format,
            active_only,
            account_type,
        } => {
            eprintln!("Fetching accounts...");
            let response = client
                .get_accounts(&user_token)
                .await
                .context("Failed to fetch accounts")?;

            let mut accounts = response.items;

            // Apply filters
            if active_only {
                accounts.retain(|a| matches!(a.status, akahu_client::Active::Active));
            }

            if let Some(ref type_filter) = account_type {
                let type_upper = type_filter.to_uppercase();
                accounts.retain(|a| format!("{:?}", a.kind).to_uppercase() == type_upper);
            }

            if accounts.is_empty() {
                eprintln!("No accounts found matching the specified filters.");
                return Ok(());
            }

            // Display results
            match format.as_str() {
                "json" => {
                    let json = output::format_json(&accounts)?;
                    println!("{}", json);
                }
                "csv" => {
                    let csv = output::format_accounts_csv(&accounts);
                    print!("{}", csv);
                }
                "table" => {
                    let table = output::format_accounts_table(&accounts);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }

        Commands::Get { account_id, format } => {
            let account_id = AccountId::new(&account_id)
                .context("Invalid account ID format. Expected format: acc_...")?;

            eprintln!("Fetching account {}...", account_id.as_str());
            let response = client
                .get_account(&user_token, &account_id)
                .await
                .with_context(|| format!("Failed to fetch account {}", account_id.as_str()))?;

            let account = response.item;

            // Display results
            match format.as_str() {
                "json" => {
                    let json = output::format_json(&account)?;
                    println!("{}", json);
                }
                "table" => {
                    let table = output::format_single_account_table(&account);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }

        Commands::Revoke { account_id, yes } => {
            let account_id = AccountId::new(&account_id)
                .context("Invalid account ID format. Expected format: acc_...")?;

            // Confirmation prompt unless --yes flag is provided
            if !yes {
                use std::io::{self, Write};
                eprintln!(
                    "Are you sure you want to revoke access to account {}?",
                    account_id.as_str()
                );
                eprintln!("This will remove the app's access to this account.");
                eprint!("Type 'yes' to confirm: ");

                io::stderr().flush()?;

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .context("Failed to read user input")?;

                if input.trim().to_lowercase() != "yes" {
                    eprintln!("Revoke cancelled.");
                    return Ok(());
                }
            }

            eprintln!("Revoking access to account {}...", account_id.as_str());
            client
                .revoke_account_access(&user_token, &account_id)
                .await
                .with_context(|| {
                    format!("Failed to revoke access to account {}", account_id.as_str())
                })?;

            eprintln!("✓ Access revoked for account {}", account_id.as_str());
        }
    }

    Ok(())
}
