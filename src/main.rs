use clap::Parser;
use reqwest;
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
    let trading_212_account = format!("{trading_212_base_api}equity/account/cash");

    let client = overseer::Trading212::new(&trading_212_base_api, &api_key).client;
    let res = client.get(trading_212_account)
        .header("Authorization", api_key)
        .send()
        .await?
        .text()
        .await?;
    println!("The response was {}", res);
    Ok(())
    
}




