use overseer::{
    overseer::enums::TransactionType,
    trading212_api::csv_parser::parse_from_csv,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    match parse_from_csv(file_path) {
        Ok(transactions) => {
            println!("--- All Transactions ---");
            for transaction in &transactions {
                println!("{:?}", transaction);
            }

            println!("\n--- Dividend Transactions ---");
            for transaction in &transactions {
                if transaction.transaction_type == TransactionType::Dividend {
                    println!("{:?}", transaction);
                }
            }

            println!("\n--- Withdrawal Transactions ---");
            for transaction in &transactions {
                if transaction.transaction_type == TransactionType::Withdrawal {
                    println!("{:?}", transaction);
                }
            }

            println!("\n--- Deposit Transactions ---");
            for transaction in &transactions {
                if transaction.transaction_type == TransactionType::Deposit {
                    println!("{:?}", transaction);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing CSV: {}", e);
        }
    }
}
