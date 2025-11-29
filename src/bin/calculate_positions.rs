use overseer::{
    overseer::{
        enums::{Asset, TransactionType},
        structs::{HistoricalTransaction, Position},
    },
    trading212_api::csv_parser::parse_from_csv,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    match parse_from_csv(file_path) {
        Ok(transactions) => {
            let positions = calculate_positions(transactions);
            print_positions(positions);
        }
        Err(e) => {
            eprintln!("Error parsing CSV: {}", e);
        }
    }
}

fn calculate_positions(transactions: Vec<HistoricalTransaction>) -> HashMap<String, Position> {
    let mut positions: HashMap<String, Position> = HashMap::new();

    for transaction in transactions {
        let ticker = match &transaction.asset {
            Asset::Stock(stock) => stock.ticker.clone(),
            Asset::Crypto(crypto) => crypto.symbol.clone(),
        };

        let position = positions.entry(ticker.clone()).or_insert_with(|| Position {
            source: "Trading 212".to_string(),
            asset: transaction.asset.clone(),
            total_value: Decimal::ZERO,
            total_cost: Decimal::ZERO,
            current_price: Decimal::ZERO,
            ppl: Decimal::ZERO,
            ppl_as_perc: Decimal::ZERO,
            quantity: Decimal::ZERO,
        });

        match transaction.transaction_type {
            TransactionType::Buy => {
                position.quantity += transaction.quantity;
                position.total_cost += transaction.total_value;
            }
            TransactionType::Sell => {
                if position.quantity > Decimal::ZERO {
                    let average_cost = position.total_cost / position.quantity;
                    let cost_of_sale = transaction.quantity * average_cost;
                    position.quantity -= transaction.quantity;
                    position.total_cost -= cost_of_sale;

                    if position.quantity.is_zero() {
                        position.total_cost = Decimal::ZERO;
                    }
                }
            }
            _ => {}
        }
    }

    positions
}

fn print_positions(positions: HashMap<String, Position>) {
    println!(
        "{:<10} | {:<15} | {:<15} | {:<15}",
        "Ticker", "Quantity", "Total Cost", "Average Price"
    );
    println!("{:-<65}", "");

    for (ticker, position) in positions {
        if position.quantity > Decimal::ZERO {
            let average_price = if position.quantity != Decimal::ZERO {
                position.total_cost / position.quantity
            } else {
                Decimal::ZERO
            };
            println!(
                "{:<10} | {:<15.4} | {:<15.2} | {:<15.4}",
                ticker,
                position.quantity,
                position.total_cost,
                average_price
            );
        }
    }
}
