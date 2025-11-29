use crate::overseer::{
    enums::{Asset, Stock, TransactionType},
    structs::HistoricalTransaction,
};
use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::{error::Error, fs::File};

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Action")]
    action: String,
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "ISIN")]
    isin: Option<String>,
    #[serde(rename = "Ticker")]
    ticker: Option<String>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "No. of shares")]
    shares: Option<Decimal>,
    #[serde(rename = "Price / share")]
    price: Option<Decimal>,
    #[serde(rename = "Total")]
    total: Option<Decimal>,
}

pub fn parse_from_csv(file_path: &str) -> Result<Vec<HistoricalTransaction>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut transactions = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        let transaction_type = match record.action.as_str() {
            "Market buy" | "Limit buy" => TransactionType::Buy,
            "Market sell" | "Limit sell" => TransactionType::Sell,
            "Deposit" => TransactionType::Deposit,
            "Withdrawal" => TransactionType::Withdrawal,
            "Dividend (Dividend)" => TransactionType::Dividend,
            _ => continue, // Skip other transaction types for now
        };

        let date = NaiveDateTime::parse_from_str(&record.time, "%Y-%m-%d %H:%M:%S")?;
        let date_utc = date.and_utc();

        let asset = if let (Some(ticker), Some(isin), Some(name)) =
            (record.ticker, record.isin, record.name)
        {
            Some(Asset::Stock(Stock {
                ticker,
                isin,
                name,
            }))
        } else {
            None
        };

        if let Some(asset) = asset {
            let transaction = HistoricalTransaction {
                asset,
                date: date_utc,
                unit_price: record.price.unwrap_or_default(),
                quantity: record.shares.unwrap_or_default(),
                total_value: record.total.unwrap_or_default(),
                transaction_type,
            };
            transactions.push(transaction);
        }
    }

    Ok(transactions)
}
