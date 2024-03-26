use overseer::trading212_api::Trading212;
use overseer::hl_client::{self, HL};
use clap::Parser;
use anyhow::Result;
use tokio;
use std::env;
use dotenv;

#[derive(clap::Parser)]  
#[command(author, version, about, long_about = None)]
struct Cli {
    username: Option<String>,
    dob: Option<String>,
    api_key: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();
    let username = &args.username;
    let dob = &args.dob;
    let api_key = &args.api_key;
    
    let env_args = dotenv::dotenv().unwrap();
    let username = env::var("HL_USERNAME").unwrap();
    let date_of_birth = env::var("HL_DATE_OF_BIRTH").unwrap();
    let password = env::var("HL_PASSWORD").unwrap();
    let secure_number = env::var("HL_SECURE_NUMBER").unwrap();

    let base_url = "https://online.hl.co.uk";
    let hl = HL::new();
    hl.login(username, date_of_birth, password, secure_number).await;
    let accounts = hl.fetch_accounts().await;
    let portfolio_positions = hl.fetch_portfolio_position().await;
    let historical_transactions = hl.fetch_all_historical_transactions(portfolio_positions).await;

    let vector = hl.fetch_portfolio_position().await;
    if let Some(api_key) = &args.api_key{
        let trading_212_base_api = "https://live.trading212.com/api/v0/".to_string();

        let client = Trading212::new(&trading_212_base_api, &api_key);
        
        println!("Vendor: Trading 212");
        
        let account_info = client.fetch_account_metadata().await;        
        
        println!("Account info:\n\tid: {}\tActive currency: {}",account_info.id, account_info.currency_code);
       
        println!("Account overview:");

        let account_liquidity = client.fetch_account_cash().await;

        println!("\tBlocked funds: {},\tFree funds: {},\t Invested: {},\n\tInvested in Pies: {},\tppl: {}\n\tTotal profit/loss on account: {},\tTotal account value: {}", 
                 account_liquidity.blocked.unwrap_or(0 as f32),
                 account_liquidity.free,
                 account_liquidity.invested,
                 account_liquidity.pie_cash,
                 account_liquidity.ppl,
                 account_liquidity.result,
                 account_liquidity.total
                 );
        
        println!("Assets:");

        let account_holdings: Vec<_> = client.fetch_portfolio_positions()
                .await    
                .into_iter()
                .map(|e| e)
                .collect();


        if account_holdings.is_empty() {
            println!("\tThere are no holdings in this account!");
        } else {
            for position in account_holdings {
                let fx_impact = if let Some(float) = position.fx_ppl {
                    float.to_string()
                } else {
                    "N/A".to_string()
                };

                println!("Ticker: {},\t Quantity: {},\t Average Price: {}\nProfit/Loss: {},\t Current Price: {},\t Forex Impact: {}.", 
                    position.ticker, 
                    position.quantity, 
                    position.average_price, 
                    position.ppl, 
                    position.current_price, 
                    fx_impact
                )
            }
        };
    }
    Ok(())
    
}




