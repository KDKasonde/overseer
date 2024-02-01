use serde::Deserialize;


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
