use crate::overseer::{
    enums::{Asset, Stock, TransactionType},
    structs::HistoricalTransaction,
};
use anyhow::{Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use pdf_extract::extract_text;
use regex::Regex;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use std::path::Path;

// --- Helper Functions ---

/// Parses financial strings in format "1,234.56" or "(123.45)".
fn parse_financial(input: &str) -> Result<Decimal> {
    let s = input.trim(); // Corrected: use input.trim()
    if s == "-" || s.is_empty() {
        return Ok(Decimal::ZERO);
    }

    let is_negative = s.starts_with('(') && s.ends_with(')');
    // Remove chars that interfere with parsing
    let clean = s.replace(&['(', ')', ',', '£'][..], "");

    let mut val = Decimal::from_str(&clean)
        .with_context(|| format!("Failed to parse decimal from '{}'", input))?;

    if is_negative {
        val = -val;
    }
    Ok(val)
}

// --- Core Parsing Logic ---

pub fn parse_hl_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<HistoricalTransaction>> {
    // 1. Ingestion
    let raw_text = extract_text(path.as_ref())
        .with_context(|| format!("Failed to extract text from {:?}", path.as_ref()))?;

    // 2. Sanitization (The Flattening)
    let mut clean_lines = Vec::new();
    let mut in_table = false;

    for line in raw_text.lines() {
        let t = line.trim();

        // Start of the relevant section - look for transaction table headers
        if t.contains("Transaction Details") ||
           (t.contains("LIFETIME ISA") && !t.contains("DETAILED VALUATION")) {
            in_table = true;
            continue;
        }

        // End of the relevant section - multiple possible markers
        if t.contains("CAPITAL ACCOUNT TRANSACTIONS")
            || t.contains("INCOME ACCOUNT TRANSACTIONS")
            || t.contains("BENEFITS OF BEING WITH HL")
            || t.contains("Venue of Execution")
            || t.contains("Code Description")
            || t.starts_with("The suggested minimum")
            || t.starts_with("The capital account") {
            in_table = false;
        }

        if in_table {
            // Filter out noise
            if t.is_empty()
                || t.starts_with("PAGE")
                || t.contains("Continued from")
                || t.contains("Transaction Date")
                || t.contains("Balance Brought Forward")
                || t.contains("Trade Type")
                || t.contains("Sedol Venue")
                || t == "Value"
                || t == "£"
                || t == "Units"
                || t.contains("Bought/Sold")
                || t.contains("Unit Price")
                || t == "Pence"
                || t == "Code"
                || t.starts_with("-")
            {
                continue;
            }
            clean_lines.push(line);
        }
    }
    // 3. Segmentation - find all transaction markers in the full text
    let full_text = clean_lines.join("\n");
    let mut transactions = Vec::new();

    // Pattern to find transaction date/type: "DD/MM/YYYY Bought|Sold"
    // This can appear at the start of a line or embedded within a line
    let txn_pattern = Regex::new(r"\d{2}/\d{2}/\d{4}\s+(Bought|Sold)")?;

    // Find all matches
    let matches: Vec<_> = txn_pattern.find_iter(&full_text).collect();

    for (idx, match_item) in matches.iter().enumerate() {
        let match_start = match_item.start();
        let match_end = match_item.end();

        // Find the start of this transaction block by looking backwards for the units line (starts with dot)
        let mut block_start = match_start.saturating_sub(200);
        if let Some(units_pos) = full_text[block_start..match_start].rfind(|c| c == '.') {
            // Find the start of the line that contains the dot
            if let Some(line_start) = full_text[..block_start + units_pos].rfind('\n') {
                block_start = line_start + 1;
            }
        }

        // Find the end of this transaction: look for "UT" after the "Bought" keyword
        // Stop at the next transaction's units line (next dot at line start) or end of text
        let after_bought = &full_text[match_end..];
        let mut block_end = full_text.len();

        // First, find the "UT" marker for this transaction
        if let Some(ut_pos) = after_bought.find("UT") {
            let tentative_end = match_end + ut_pos + 2; // +2 for "UT"

            // Now look for the next transaction (next line starting with dot)
            if idx + 1 < matches.len() {
                // Use the next match as the boundary
                block_end = matches[idx + 1].start();
            } else {
                block_end = tentative_end + 20; // Add some buffer
            }
        }

        let block = &full_text[block_start..block_end.min(full_text.len())];

        match parse_single_transaction(block) {
            Ok(txn) => transactions.push(txn),
            Err(e) => eprintln!("Failed to parse transaction:\n{}\nError: {}", &block[..block.len().min(300)], e),
        }
    }

    // Also handle Management Fee and Interest transactions
    let fee_pattern = Regex::new(r"\([\d.]+\)\s*Management Fee")?;
    let interest_pattern = Regex::new(r"[\d.]+\s*Interest\s+:")?;

    for pattern in [fee_pattern, interest_pattern] {
        for match_item in pattern.find_iter(&full_text) {
            let match_start = match_item.start();
            let match_end = match_item.end();

            // Look forward to find the date and balance
            let block_end = full_text[match_end..]
                .char_indices()
                .filter(|(_, c)| *c == '\n')
                .nth(1)
                .map(|(pos, _)| match_end + pos)
                .unwrap_or(full_text.len());

            let block = &full_text[match_start..block_end];

            match parse_single_transaction(block) {
                Ok(txn) => transactions.push(txn),
                Err(e) => eprintln!("Failed to parse fee/interest:\n{}\nError: {}", block, e),
            }
        }
    }

    if transactions.is_empty() {
        eprintln!("No transactions found. Sample text (first 500 chars):");
        eprintln!("{}", &full_text[..full_text.len().min(500)]);
    }

    Ok(transactions)
}

fn parse_single_transaction(block: &str) -> Result<HistoricalTransaction> {
    let full_str = block.replace("\n", " ");

    // Find the date (format: DD/MM/YYYY)
    let date_re = Regex::new(r"\d{2}/\d{2}/\d{4}").unwrap();
    let date_match = date_re.find(&full_str)
        .ok_or_else(|| anyhow::anyhow!("No date found in block"))?;
    let date_str = date_match.as_str();
    let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y")?;

    // Determine transaction type
    let trade_type = if full_str.contains("Bought") {
        TransactionType::Buy
    } else if full_str.contains("Sold") {
        TransactionType::Sell
    } else if full_str.contains("Management Fee") {
        TransactionType::Fee
    } else if full_str.contains("Interest") {
        TransactionType::Deposit
    } else {
        TransactionType::Unknown
    };

    let mut units = Decimal::ZERO;
    let mut price = Decimal::ZERO;
    let mut value = Decimal::ZERO;
    let mut sedol = String::new();
    let mut fund_name = String::new();

    if trade_type == TransactionType::Buy || trade_type == TransactionType::Sell {
        // Parse Buy/Sell transactions
        // Format: .units price (value) FundName @ venue DD/MM/YYYY Bought/Sold balance SEDOL venue

        // Extract the main transaction line which starts with units
        // HL Format: .IIIDDD where last 3 digits are decimals, rest are integers
        // Examples: .7170 -> 7.170, .07769 -> 69.077, .963130 -> 963.130
        let txn_line_re = Regex::new(r"\.(\d+)\s+([\d,]+\.?\d*)\s+\(([\d,]+\.?\d*)\)").unwrap();
        if let Some(cap) = txn_line_re.captures(&full_str) {
            let all_digits = cap.get(1).unwrap().as_str();

            // Split based on digit length:
            // 4 digits: first 2 decimal, last 2 integer (.7170 -> 70.71)
            // 5+ digits: first 3 decimal, rest integer (.07769 -> 69.077)
            // 3 or fewer: all decimal (0.XXX)
            if all_digits.len() == 4 {
                let decimal_part = &all_digits[..2];
                let integer_part = &all_digits[2..];
                let units_str = format!("{}.{}", integer_part, decimal_part);
                units = parse_financial(&units_str)?;
            } else if all_digits.len() > 4 {
                let decimal_part = &all_digits[..3];
                let integer_part = &all_digits[3..];
                let units_str = format!("{}.{}", integer_part, decimal_part);
                units = parse_financial(&units_str)?;
            } else {
                // 3 or fewer digits, all decimal (e.g., .123 -> 0.123)
                let units_str = format!("0.{}", all_digits);
                units = parse_financial(&units_str)?;
            }

            // Price: 33,462.99 (in pence)
            price = parse_financial(cap.get(2).unwrap().as_str())?;

            // Value: (240.00) - negative because money going out
            value = parse_financial(cap.get(3).unwrap().as_str())?;
            value = -value;
        }

        // Extract SEDOL (7-character alphanumeric code before "UT")
        // The SEDOL appears right after "Bought balance" in the format: "Bought 123.45SEDOL UT" or "Bought 123.45 SEDOL UT"
        // Extract it by looking for the pattern: Bought, then numbers, then 7-char code, then UT
        let sedol_re = Regex::new(r"Bought\s+([\d,.]+)([A-Z0-9]{7})\s*UT\b").unwrap();
        if let Some(cap) = sedol_re.captures(&full_str) {
            sedol = cap.get(2).unwrap().as_str().to_string();
        } else {
            // Fallback: look for SEDOL with a space between balance and SEDOL
            let sedol_re2 = Regex::new(r"Bought\s+([\d,.]+)\s+([A-Z][A-Z0-9]{6})\s*UT\b").unwrap();
            if let Some(cap) = sedol_re2.captures(&full_str) {
                sedol = cap.get(2).unwrap().as_str().to_string();
            }
        }

        // Extract fund name
        // It's between the closing parenthesis of the value and the @ symbol
        // The fund name may be followed by "@ venue" or directly by "venue-code date"

        // First, try the standard format: ") FundName @ venue"
        let name_re = Regex::new(r"\)\s*([^@]+?)\s+@").unwrap();
        if let Some(cap) = name_re.captures(&full_str) {
            fund_name = cap.get(1).unwrap().as_str().trim().to_string();
        } else {
            // Fallback for concatenated format: ")FundName @ venue-codeDD/MM/YYYY"
            // Extract everything between ) and @ symbol
            let name_re2 = Regex::new(r"\)([^@]*?)@").unwrap();
            if let Some(cap) = name_re2.captures(&full_str) {
                fund_name = cap.get(1).unwrap().as_str().trim().to_string();
            } else {
                // Last resort: extract between ) and a 4-digit venue code
                let name_re3 = Regex::new(r"\)\s*(.+?)\s+\d{4}\d{2}/\d{2}/\d{4}").unwrap();
                if let Some(cap) = name_re3.captures(&full_str) {
                    fund_name = cap.get(1).unwrap().as_str().trim().to_string();
                }
            }
        }

        // Clean up fund name - remove venue codes if they snuck in
        if fund_name.is_empty() || fund_name == "@" {
            // Try to extract from context if fund name extraction failed
            fund_name = "Unknown Fund".to_string();
        }
    } else if trade_type == TransactionType::Fee {
        // Management Fee format: (value) Management Fee : Description date balance
        let value_re = Regex::new(r"\(([\d,\.]+)\)").unwrap();
        if let Some(cap) = value_re.captures(&full_str) {
            value = parse_financial(cap.get(1).unwrap().as_str())?;
            value = -value; // Fees are negative
        }
        fund_name = "Management Fee".to_string();
    } else if trade_type == TransactionType::Deposit {
        // Interest format: value Interest : From date TO date date balance
        let value_re = Regex::new(r"([\d,\.]+)\s*Interest").unwrap();
        if let Some(cap) = value_re.captures(&full_str) {
            value = parse_financial(cap.get(1).unwrap().as_str())?;
        }
        fund_name = "Interest Payment".to_string();
    }

    let asset = Asset::Stock(Stock {
        ticker: if sedol.is_empty() { "N/A".to_string() } else { sedol },
        isin: "".to_string(),
        name: fund_name,
    });

    Ok(HistoricalTransaction {
        date: Utc.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap()).unwrap(),
        transaction_type: trade_type,
        asset,
        quantity: units,
        unit_price: price,
        total_value: value,
    })
}