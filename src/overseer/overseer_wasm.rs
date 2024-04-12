use std::future::IntoFuture;

use wasm_bindgen::prelude::*;
use serde::Deserialize;

use crate::hl_client::HL;
use crate::overseer::structs::{Account, Position, HistoricalTransaction};
use crate::overseer::traits::OverseenAccount;
use crate::overseer::errors::OverseerError;
use crate::trading212_api::Trading212;


#[wasm_bindgen]
pub struct OverSeen { 
     overseen_account: Box<dyn OverseenAccount>,
     vendor: String
}

#[wasm_bindgen]
impl OverSeen {
    #[wasm_bindgen(constructor)]
    pub fn new(vendor: &str, base_url: Option<String>, api_key: Option<String>) -> OverSeen {
        match vendor {
            "trading212" => {
                let api_key = if let Some(key) = api_key {
                    key 
                } else { 
                    panic!("no api key found but api key required")
                };

                let base_url = if let Some(url) = base_url {
                    url
                } else {
                    panic!("no url given but it is required")
                };

                OverSeen {
                    overseen_account: Trading212::new(base_url.into(), api_key.into(),),
                    vendor: "trading212".to_string()
                }
            },
            "hl"=> {
                OverSeen {
                    overseen_account: HL::new(),
                    vendor: "hl".to_string()
                }
            }
        }
    }

    pub fn authenticate(self, username: String, date_of_birth: String, password: String, secure_number: String) {
        match self.vendor.as_ref() {
            "hl" => {
                self.overseen_account.login(
                    username= Some(username), 
                    password= Some(password), 
                    date_of_birth= Some(date_of_birth), 
                    secure_number= Some(secure_number)
                )
            },
            _ => {
                panic!("login not required")
            }
        }
    }

    pub fn get_accounts(&self) -> Result<Vec<Result<Account,OverseerError>>,OverseerError> {
        self.overseen_account.get_cash()
    }
     
    pub async fn get_vendor_summary(&self) -> Result<Account, OverseerError> {
        let all_accounts = match self.overseen_account.get_cash().await {
            Ok(accounts) => {
                accounts
            },
            Err(OverseerError) => {
                return OverseerError
            }
        };
        let mut overview = Account {
           vendor : self.vendor,
           blocked : 0., 
           free : 0.,
           total_funds : 0.,
           invested : 0.,
           ppl : 0., 
           total : 0. 
        };

        let account = all_accounts
            .iter()
            .map(
                |account| {
                    let account = match account {
                        Ok(account_data) => {
                            overview += account_data;

                        },
                        Err(e) => {
                            return e
                        }
                    };
                }
                )
            .collect();

        Ok(overview)
    }
}


