use super::Trading212;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct DividendDetails {
    gained: f32,
    in_cash: f32,
    reinvested: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct InvestmentResult {
    invested_value: f32,
    result: f32,
    result_coef: f32,
    value: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Issue {
    name: String,
    severity: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct PieMetadata {
    cash: f32,
    id: i64,
    progress: f32,
    result: InvestmentResult,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct InstrumentDetails {
    current_share: f32,
    expected_share: f32,
    issues: Vec<Issue>,
    owned_quantity: f32,
    result: Vec<InvestmentResult>,
    ticker: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct AccountBucketDetailedResponse {
    creation_date: String,
    dividend_cash_action: String,
    end_date: String,
    goal: f32,
    icon: String,
    id: i64,
    initial_investment: f32,
    name: String,
    pubic_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct PieList {
    pies: Vec<PieMetadata>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Pie {
    instruments: Vec<InstrumentDetails>,
    settings: Vec<AccountBucketDetailedResponse>,
}

impl Trading212 {
    
    pub async fn fetch_all_pies_info(&self) -> PieList {
        let client = &self.client;
        let target_url = format!("{}equity/pies", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<PieList>()
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

    pub async fn fetch_pie(&self, id: i64) -> Pie {
        let client = &self.client;
        let target_url = format!("{}equity/account/portfolio/{id}", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<Pie>()
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
