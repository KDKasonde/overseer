use crate::overseer::traits::OverseerError;

use super::Trading212;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cash {
    pub blocked: Option<f32>,
    pub free: f32,
    pub invested: f32,
    pub pie_cash: f32,
    pub ppl: f32,
    pub result: f32,
    pub total: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub currency_code: String,
    pub id: u64,
}

impl Trading212 {
    
    pub async fn fetch_account_cash(&self) -> Result<Cash, OverseerError> {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/cash", self.base_url );

        let res = client
            .get(&target_url)
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<Cash>()
                    .await
            },
            Err(..)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                return Err(OverseerError::FailedFetch { url: target_url })    
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
        
        Ok(output)
        
    }

    pub async fn fetch_account_metadata(&self) -> Metadata {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/info", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
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
