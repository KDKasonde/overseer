use serde::Deserialize;
use reqwest::{
    Client,
    header,
};

//let trading_212_base_api = "https://live.trading212.com/api/v0/".to_string();
//let trading_212_account = format!("{trading_212_base_api}equity/account/cash");
//let client = reqwest::blocking::Client::new();
 
pub struct Trading212 {
    
    client: Client,
    base_url: String,
    
}

#[derive(Debug, Deserialize)]
pub struct Cash {
    blocked: f32,
    free: f32,
    invested: f32,
    pieCash: f32,
    ppl: f32,
    result: f32,
    total: f32,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    currencyCode: String,
    id: u64,
}

#[derive(Debug, Deserialize)]
pub struct OpenPosition {
    averagePrice: f32,
    currentPrice: f32,
    frontend: String,
    fxPpl: f32,
    initialFillDate: String,
    maxBuy: f32,
    maxSell: f32,
    pieQuantity: f32,
    ppl: f32,
    quantity: f32,
    ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct AllOpenPositions {
    positions: Vec<Option<OpenPosition>>,
}



impl Trading212 {
    
    pub fn new(base_url: &str, api_key: &str) -> Trading212 {
        let mut headers = header::HeaderMap::new();

        headers.insert("User-Agent", header::HeaderValue::from_static("OverSeer"));

        let mut auth_value = header::HeaderValue::from_str(api_key).unwrap();
        
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
                       
        let client_builder = Client::builder()
            .default_headers(headers)
            .build();
        
        let client = match client_builder {
            Ok(client) => client,
            Err(_error) => panic!("Error creating client instance!")
        };

        Trading212 {
            client: client,
            base_url: base_url.to_string()
        }
    }

    pub async fn fetch_account_cash(&self) -> Result<Cash, reqwest::Error> {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/cash", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
            Ok(response) => { 
                response
                    .json::<Cash>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        return output
        
    }

    pub async fn fetch_account_metadata(&self) -> Result<Metadata, reqwest::Error> {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/info", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
            Ok(response) => { 
                response
                    .json::<Metadata>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        return output

    }
    
    pub async fn fetch_portfolio_positions(&self) -> Result<AllOpenPositions, reqwest::Error>{
        
        let client = &self.client;
        let target_url = format!("{}equity/portfolio", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
            Ok(response) => { 
                response
                    .json::<AllOpenPositions>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
            
        return output

    }
    pub async fn fetch_position(&self, ticker: &str) -> Result<OpenPosition, reqwest::Error> {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/portfolio/{ticker}", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
            Ok(response) => { 
                response
                    .json::<OpenPosition>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 

        return output

    }
}


