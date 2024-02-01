use serde::Deserialize;

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

impl Trading212 {
    
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
}
