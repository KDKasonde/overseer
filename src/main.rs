use overseer::trading212_api::Trading212;
use clap::Parser;
use anyhow::Result;
use tokio;

#[derive(clap::Parser)]  
#[command(author, version, about, long_about = None)]
struct Cli {
    api_key: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();
    let api_key = &args.api_key;

    let trading_212_base_api = "https://live.trading212.com/api/v0/".to_string();

    let client = Trading212::new(&trading_212_base_api, &api_key);
    let res = client.fetch_account_metadata()
        .await;


    let res = client.fetch_portfolio_positions()
        .await;
    match res {
        Ok(_) => println!("ran successfully!"),
        Err(e) => println!("Error {} occured", e),
    }

    Ok(())
    
}




