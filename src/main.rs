use overseer::trading212_api::Trading212;
use overseer::hl_client::{self, login_step_one};
use clap::Parser;
use anyhow::Result;
use tokio;

#[derive(clap::Parser)]  
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
    dob: String,
    api_key: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();
    let username = &args.username;
    let dob = &args.dob;
    let api_key = &args.api_key;
    
    let base_url = "https://online.hl.co.uk/my-accounts/login-step-one";

    login_step_one(base_url, username, dob).await;

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




