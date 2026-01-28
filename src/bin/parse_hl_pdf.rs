use overseer::hl_client::pdf_parser::parse_hl_pdf;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf_file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    match parse_hl_pdf(file_path) {
        Ok(transactions) => {
            println!("Successfully parsed {} transactions from HL PDF:\n", transactions.len());

            for (idx, txn) in transactions.iter().enumerate() {
                println!("Transaction #{}", idx + 1);
                println!("  Date: {}", txn.date.format("%Y-%m-%d"));
                println!("  Type: {:?}", txn.transaction_type);
                println!("  Asset: {:?}", txn.asset);
                println!("  Quantity: {}", txn.quantity);
                println!("  Unit Price: {}", txn.unit_price);
                println!("  Total Value: {}", txn.total_value);
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error parsing HL PDF: {}", e);
            std::process::exit(1);
        }
    }
}
