use super::Trading212;

use serde::{de::Error, Deserialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    pub average_price: f32,
    pub current_price: f32,
    pub frontend: String,
    pub fx_ppl: Option<f32>,
    pub initial_fill_date: String,
    pub max_buy: f32,
    pub max_sell: f32,
    pub pie_quantity: f32,
    pub ppl: f32,
    pub quantity: f32,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOpenPositions {
    pub positions: Option<Vec<Option<OpenPosition>>>,
}

impl Trading212 {
    pub async fn fetch_portfolio_positions(&self) -> Vec<OpenPosition> {
        let client = &self.client;
        let target_url = format!("{}equity/portfolio", self.base_url );

        let res = client
            .get(&target_url)
            .send()
            .await;
        
        
        let res: Vec<OpenPosition> = match res {
            Ok(response) => {
                let response = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "[]".to_string());

                serde_json::from_str(&response).unwrap()
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        let output = res;    
        return output
    }
    
    pub async fn fetch_position(&self, ticker: &str) -> OpenPosition {
        let client = &self.client;
        let target_url = format!("{}equity/account/portfolio/{ticker}", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
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
        
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        
        return output
    }
}
