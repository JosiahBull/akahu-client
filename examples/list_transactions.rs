//! List transactions for a specific account within a date range.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example list_transactions
//! ```

use akahu_client::{AccountId, AkahuClient, UserToken};
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use clap::Parser;

/// List transactions for a specific account
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// User access token (can also be set via AKAHU_USER_TOKEN env var)
    #[arg(short = 'u', long, env = "AKAHU_USER_TOKEN")]
    user_token: String,

    /// Application ID token (can also be set via AKAHU_APP_TOKEN env var)
    #[arg(short = 'a', long, env = "AKAHU_APP_TOKEN")]
    app_token: String,

    /// Account ID to fetch transactions for (prefixed with "acc_")
    #[arg(short = 'i', long, help = "Account ID (e.g., acc_123...)")]
    account_id: String,

    /// Start date (exclusive) in YYYY-MM-DD format
    #[arg(
        short = 's',
        long,
        help = "Start date (exclusive) in YYYY-MM-DD format"
    )]
    start: Option<String>,

    /// End date (inclusive) in YYYY-MM-DD format
    #[arg(short = 'e', long, help = "End date (inclusive) in YYYY-MM-DD format")]
    end: Option<String>,

    /// Fetch all pages (may take a while for large datasets)
    #[arg(long, help = "Fetch all pages instead of just the first page")]
    all_pages: bool,

    /// Output format
    #[arg(short = 'f', long, default_value = "table", value_parser = ["table", "json", "csv"])]
    format: String,

    /// Maximum number of pages to fetch (when using --all-pages)
    #[arg(long, default_value = "10", help = "Maximum pages to fetch")]
    max_pages: usize,
}

fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    let naive = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .with_context(|| format!("Invalid date format: '{}'. Expected YYYY-MM-DD", date_str))?;

    Ok(naive.and_hms_opt(0, 0, 0).expect("Valid time").and_utc())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Create the Akahu client
    let client = AkahuClient::new(reqwest::Client::new(), args.app_token, None);

    let user_token = UserToken::new(args.user_token);
    let account_id = AccountId::new(&args.account_id).context("Invalid account ID format")?;

    // Parse dates if provided
    let start_date = args.start.as_ref().map(|s| parse_date(s)).transpose()?;
    let end_date = args.end.as_ref().map(|s| parse_date(s)).transpose()?;

    // Build query parameters
    let query = (start_date, end_date, None);

    // Fetch transactions
    println!(
        "Fetching transactions for account {}...",
        account_id.as_str()
    );

    let mut all_transactions = Vec::new();
    let mut current_query = Some(query);
    let mut page_count = 0;

    while let Some(q) = current_query {
        if page_count >= args.max_pages {
            println!(
                "\nReached maximum page limit ({}). Use --max-pages to fetch more.",
                args.max_pages
            );
            break;
        }

        let response = client
            .get_account_transactions(&user_token, &account_id, q.0, q.1, q.2)
            .await
            .context("Failed to fetch transactions")?;

        let transaction_count = response.items.len();
        all_transactions.extend(response.items);
        page_count = page_count.saturating_add(1);

        println!(
            "  Fetched page {}: {} transactions",
            page_count, transaction_count
        );

        // Check if there are more pages
        if args.all_pages && response.cursor.next.is_some() {
            // Build query for next page
            current_query = Some((start_date, end_date, response.cursor.next));
        } else {
            current_query = None;
        }
    }

    if all_transactions.is_empty() {
        println!("No transactions found.");
        return Ok(());
    }

    // Display results
    match args.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&all_transactions)
                .context("Failed to serialize to JSON")?;
            println!("{}", json);
        }
        "csv" => {
            // CSV header
            println!("Date,Description,Amount,Balance,Type");
            for tx in &all_transactions {
                println!(
                    "{},{},{},{},{:?}",
                    tx.date.format("%Y-%m-%d"),
                    tx.description.replace(',', ";"), // Escape commas
                    tx.amount,
                    tx.balance
                        .as_ref()
                        .map(|b| b.to_string())
                        .unwrap_or_default(),
                    tx.kind
                );
            }
        }
        "table" => {
            println!("\n{} transaction(s) found:\n", all_transactions.len());
            println!(
                "{:<12} {:<50} {:<15} {:<15} {:<15}",
                "DATE", "DESCRIPTION", "AMOUNT", "BALANCE", "TYPE"
            );
            println!("{}", "=".repeat(107));

            for tx in &all_transactions {
                println!(
                    "{:<12} {:<50} {:>15.2} {:>15} {:<15}",
                    tx.date.format("%Y-%m-%d"),
                    tx.description.chars().take(50).collect::<String>(),
                    tx.amount,
                    tx.balance
                        .as_ref()
                        .map(|b| format!("{:.2}", b))
                        .unwrap_or_else(|| "-".to_string()),
                    format!("{:?}", tx.kind)
                );
            }

            // Summary
            println!("\n{}", "=".repeat(107));
            let total: rust_decimal::Decimal = all_transactions.iter().map(|t| t.amount).sum();
            println!("Total: {:.2}", total);
        }
        _ => unreachable!(),
    }

    Ok(())
}
