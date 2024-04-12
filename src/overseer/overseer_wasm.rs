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
                    overseen_account: Box::new(Trading212::new(base_url.as_str(), api_key.as_str())),
                    vendor: "trading212".to_string()
                }
            },
            "hl"=> {
                OverSeen {
                    overseen_account: Box::new(HL::new()),
                    vendor: "hl".to_string()
                }
            }, 
            _ => {
                panic!("The input vendor {} is not supported.", vendor)
            }
        }
    }

    pub async fn authenticate(self, username: String, date_of_birth: String, password: String, secure_number: String) {
        match self.vendor.as_ref() {
            "hl" => {
                self.overseen_account.login(
                    Some(username), 
                    Some(password), 
                    Some(date_of_birth), 
                    Some(secure_number)
                )
                .await;
            },
            _ => {
                panic!("login not required")
            }
        }
    }
     
    pub async fn get_vendor_summary(&self) -> Result<JsValue, JsValue> {
        let all_accounts = self.overseen_account.get_cash().await;

        let mut overview = Account {
           vendor : self.vendor.clone(),
           blocked : 0., 
           free : 0.,
           total_funds : 0.,
           invested : 0.,
           ppl : 0., 
           total : 0. 
        };

        let _: Vec<_> = all_accounts
            .into_iter()
            .map(
                |account| {
                    overview += &account.unwrap();
                }
            )
            .collect();

        serde_wasm_bindgen::to_value(&overview).map_err(|err| err.into())
    }
}


