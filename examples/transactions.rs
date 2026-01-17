//! Transaction query CLI example for Akahu client.
//!
//! This example demonstrates how to:
//! - List all transactions with pagination and date filtering
//! - Get pending transactions
//! - Query transactions for specific accounts
//! - Handle cursor-based pagination
//!
//! # Authentication
//!
//! Requires two tokens:
//! - User token (AKAHU_USER_TOKEN or --user-token)
//! - App token (AKAHU_APP_TOKEN or --app-token)
//!
//! # Usage Examples
//!
//! List all transactions with date range:
//! ```bash
//! cargo run --example transactions list --start 2024-01-01 --end 2024-12-31
//! ```
//!
//! Get pending transactions:
//! ```bash
//! cargo run --example transactions pending --format json
//! ```
//!
//! List transactions for specific account:
//! ```bash
//! cargo run --example transactions account --account-id acc_123... --all-pages
//! ```
//!
//! Export to CSV:
//! ```bash
//! cargo run --example transactions list --format csv > transactions.csv
//! ```

use akahu_client::{AccountId, AkahuClient, UserToken};
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};

/// Transaction query operations for Akahu
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
    /// List all transactions across all accounts
    List {
        /// Start date (exclusive) in YYYY-MM-DD format
        #[arg(short = 's', long)]
        start: Option<String>,

        /// End date (inclusive) in YYYY-MM-DD format
        #[arg(short = 'e', long)]
        end: Option<String>,

        /// Fetch all pages (may take a while for large datasets)
        #[arg(long)]
        all_pages: bool,

        /// Maximum number of pages to fetch
        #[arg(long, default_value = "10")]
        max_pages: usize,

        /// Output format
        #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
        format: String,
    },

    /// Get pending transactions (all accounts)
    Pending {
        /// Output format
        #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
        format: String,
    },

    /// Get transactions for specific account
    Account {
        /// Account ID
        #[arg(short = 'i', long)]
        account_id: String,

        /// Start date (exclusive) in YYYY-MM-DD format
        #[arg(short = 's', long)]
        start: Option<String>,

        /// End date (inclusive) in YYYY-MM-DD format
        #[arg(short = 'e', long)]
        end: Option<String>,

        /// Fetch all pages (may take a while for large datasets)
        #[arg(long)]
        all_pages: bool,

        /// Maximum number of pages to fetch
        #[arg(long, default_value = "10")]
        max_pages: usize,

        /// Output format
        #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
        format: String,
    },

    /// Get pending transactions for specific account
    AccountPending {
        /// Account ID
        #[arg(short = 'i', long)]
        account_id: String,

        /// Output format
        #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
        format: String,
    },
}

fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    let naive = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .with_context(|| format!("Invalid date format: '{}'. Expected YYYY-MM-DD", date_str))?;

    Ok(naive.and_hms_opt(0, 0, 0).expect("Valid time").and_utc())
}

mod output {
    use akahu_client::{PendingTransaction, Transaction};
    use anyhow::{Context, Result};
    use serde::Serialize;

    pub fn format_json<T: Serialize>(items: &T) -> Result<String> {
        serde_json::to_string_pretty(items).context("Failed to serialize to JSON")
    }

    pub fn format_transactions_table(transactions: &[Transaction]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "\n{} transaction(s) found:\n\n",
            transactions.len()
        ));
        output.push_str(&format!(
            "{:<12} {:<50} {:<15} {:<15} {:<15}\n",
            "DATE", "DESCRIPTION", "AMOUNT", "BALANCE", "TYPE"
        ));
        output.push_str(&format!("{}\n", "=".repeat(107)));

        for tx in transactions {
            output.push_str(&format!(
                "{:<12} {:<50} {:>15.2} {:>15} {:<15}\n",
                tx.date.format("%Y-%m-%d"),
                truncate(&tx.description, 50),
                tx.amount,
                tx.balance
                    .as_ref()
                    .map(|b| format!("{:.2}", b))
                    .unwrap_or_else(|| "-".to_string()),
                format!("{:?}", tx.kind)
            ));
        }

        // Summary
        output.push_str(&format!("\n{}\n", "=".repeat(107)));
        let total: rust_decimal::Decimal = transactions.iter().map(|t| t.amount).sum();
        output.push_str(&format!("Total: {:.2}\n", total));

        output
    }

    pub fn format_transactions_csv(transactions: &[Transaction]) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("Date,Description,Amount,Balance,Type,Account ID\n");

        // CSV rows
        for tx in transactions {
            output.push_str(&format!(
                "{},{},{},{},{:?},{}\n",
                tx.date.format("%Y-%m-%d"),
                escape_csv(&tx.description),
                tx.amount,
                tx.balance
                    .as_ref()
                    .map(|b: &rust_decimal::Decimal| b.to_string())
                    .unwrap_or_default(),
                tx.kind,
                tx.account.as_str()
            ));
        }

        output
    }

    pub fn format_pending_table(pending: &[PendingTransaction]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "\n{} pending transaction(s) found:\n\n",
            pending.len()
        ));
        output.push_str(&format!(
            "{:<12} {:<50} {:<15} {:<20}\n",
            "DATE", "DESCRIPTION", "AMOUNT", "TYPE"
        ));
        output.push_str(&format!("{}\n", "=".repeat(97)));

        for tx in pending {
            output.push_str(&format!(
                "{:<12} {:<50} {:>15.2} {:<20}\n",
                tx.date.format("%Y-%m-%d"),
                truncate(&tx.description, 50),
                tx.amount,
                format!("{:?}", tx.kind)
            ));
        }

        // Summary
        output.push_str(&format!("\n{}\n", "=".repeat(97)));
        let total: rust_decimal::Decimal = pending.iter().map(|t| t.amount).sum();
        output.push_str(&format!("Total: {:.2}\n", total));

        output
    }

    pub fn format_pending_csv(pending: &[PendingTransaction]) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("Date,Description,Amount,Type,Account ID\n");

        // CSV rows
        for tx in pending {
            output.push_str(&format!(
                "{},{},{},{:?},{}\n",
                tx.date.format("%Y-%m-%d"),
                escape_csv(&tx.description),
                tx.amount,
                tx.kind,
                tx.account.as_str()
            ));
        }

        output
    }

    fn truncate(s: &str, max_len: usize) -> String {
        s.chars().take(max_len).collect::<String>()
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
            start,
            end,
            all_pages,
            max_pages,
            format,
        } => {
            // Parse dates if provided
            let start_date = start.as_ref().map(|s| parse_date(s)).transpose()?;
            let end_date = end.as_ref().map(|s| parse_date(s)).transpose()?;

            eprintln!("Fetching transactions...");

            let mut all_transactions = Vec::new();
            let mut current_cursor = None;
            let mut page_count = 0;

            loop {
                if !all_pages && page_count >= max_pages {
                    eprintln!(
                        "\nReached page limit ({}). Use --all-pages or --max-pages to fetch more.",
                        max_pages
                    );
                    break;
                }

                let response = client
                    .get_transactions(&user_token, start_date, end_date, current_cursor)
                    .await
                    .context("Failed to fetch transactions")?;

                let transaction_count = response.items.len();
                all_transactions.extend(response.items);
                page_count = page_count.saturating_add(1);

                eprintln!(
                    "  Fetched page {}: {} transactions",
                    page_count, transaction_count
                );

                // Check if there are more pages
                if all_pages && response.cursor.next.is_some() {
                    current_cursor = response.cursor.next;
                } else {
                    break;
                }
            }

            if all_transactions.is_empty() {
                eprintln!("No transactions found.");
                return Ok(());
            }

            // Display results
            match format.as_str().to_lowercase().as_str() {
                "json" => {
                    let json = output::format_json(&all_transactions)?;
                    println!("{}", json);
                }
                "csv" => {
                    let csv = output::format_transactions_csv(&all_transactions);
                    print!("{}", csv);
                }
                "table" => {
                    let table = output::format_transactions_table(&all_transactions);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }

        Commands::Pending { format } => {
            eprintln!("Fetching pending transactions...");

            let pending = client
                .get_pending_transactions(&user_token)
                .await
                .context("Failed to fetch pending transactions")?;

            if pending.is_empty() {
                eprintln!("No pending transactions found.");
                return Ok(());
            }

            eprintln!("  Fetched {} pending transactions", pending.len());

            // Display results
            match format.as_str() {
                "json" => {
                    let json = output::format_json(&pending)?;
                    println!("{}", json);
                }
                "csv" => {
                    let csv = output::format_pending_csv(&pending);
                    print!("{}", csv);
                }
                "table" => {
                    let table = output::format_pending_table(&pending);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }

        Commands::Account {
            account_id,
            start,
            end,
            all_pages,
            max_pages,
            format,
        } => {
            let account_id = AccountId::new(&account_id)
                .context("Invalid account ID format. Expected format: acc_...")?;

            // Parse dates if provided
            let start_date = start.as_ref().map(|s| parse_date(s)).transpose()?;
            let end_date = end.as_ref().map(|s| parse_date(s)).transpose()?;

            eprintln!(
                "Fetching transactions for account {}...",
                account_id.as_str()
            );

            let mut all_transactions = Vec::new();
            let mut current_cursor = None;
            let mut page_count = 0;

            loop {
                if !all_pages && page_count >= max_pages {
                    eprintln!(
                        "\nReached page limit ({}). Use --all-pages or --max-pages to fetch more.",
                        max_pages
                    );
                    break;
                }

                let response = client
                    .get_account_transactions(
                        &user_token,
                        &account_id,
                        start_date,
                        end_date,
                        current_cursor,
                    )
                    .await
                    .context("Failed to fetch account transactions")?;

                let transaction_count = response.items.len();
                all_transactions.extend(response.items);
                page_count = page_count.saturating_add(1);

                eprintln!(
                    "  Fetched page {}: {} transactions",
                    page_count, transaction_count
                );

                // Check if there are more pages
                if all_pages && response.cursor.next.is_some() {
                    current_cursor = response.cursor.next;
                } else {
                    break;
                }
            }

            if all_transactions.is_empty() {
                eprintln!("No transactions found.");
                return Ok(());
            }

            // Display results
            match format.as_str() {
                "json" => {
                    let json = output::format_json(&all_transactions)?;
                    println!("{}", json);
                }
                "csv" => {
                    let csv = output::format_transactions_csv(&all_transactions);
                    print!("{}", csv);
                }
                "table" => {
                    let table = output::format_transactions_table(&all_transactions);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }

        Commands::AccountPending { account_id, format } => {
            let account_id = AccountId::new(&account_id)
                .context("Invalid account ID format. Expected format: acc_...")?;

            eprintln!(
                "Fetching pending transactions for account {}...",
                account_id.as_str()
            );

            let pending = client
                .get_account_pending_transactions(&user_token, &account_id)
                .await
                .context("Failed to fetch account pending transactions")?;

            if pending.is_empty() {
                eprintln!("No pending transactions found.");
                return Ok(());
            }

            eprintln!("  Fetched {} pending transactions", pending.len());

            // Display results
            match format.as_str() {
                "json" => {
                    let json = output::format_json(&pending)?;
                    println!("{}", json);
                }
                "csv" => {
                    let csv = output::format_pending_csv(&pending);
                    print!("{}", csv);
                }
                "table" => {
                    let table = output::format_pending_table(&pending);
                    print!("{}", table);
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
