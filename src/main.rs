use clap::Parser;
use reqwest;
use anyhow::{Context, Result};

#[derive(clap::Parser)]  
#[command(author, version, about, long_about = None)]
struct Cli {
    api_key: String,
}

fn main() -> Result<()> {

    let args = Cli::parse();
    let api_key = &args.api_key;

    let trading_212_base_api = "https://live.trading212.com/api/v0/".to_string();
    let trading_212_account = format!("{trading_212_base_api}equity/account/info");

    let client = reqwest::blocking::Client::new();
    let res = client.get(trading_212_account)
        .header("Authorization", api_key)
        .send()?;
    println!("The response was {}", res.status());
    Ok(())
    
}




