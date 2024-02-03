use super::Trading212;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DividendDetails {
    gained: f32,
    inCash: f32,
    reinvested: f32,
}

#[derive(Debug, Deserialize)]
pub struct InvestmentResult {
    investedValue: f32,
    result: f32,
    resultCoef: f32,
    value: f32,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    Name: String,
    Severity: String,
}

#[derive(Debug, Deserialize)]
pub struct PieMetadata {
    cash: f32,
    id: i64,
    progress: f32,
    result: InvestmentResult,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentDetails {
    currentShare: f32,
    expectedShare: f32,
    issues: Vec<Issue>,
    ownedQuantity: f32,
    result: Vec<InvestmentResult>,
    ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountBucketDetailedResponse {
    creationDate: String,
    dividendCashAction: String,
    endDate: String,
    goal: f32,
    icon: String,
    id: i64,
    initialInvestment: f32,
    name: String,
    pubicUrl: String,
}

#[derive(Debug, Deserialize)]
pub struct PieList {
    pies: Vec<PieMetadata>,
}

#[derive(Debug, Deserialize)]
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
