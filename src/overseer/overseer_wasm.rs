use wasm_bindgen::prelude::*;
use serde::Deserialize;

use crate::hl_client::HL;
use crate::overseer::structs::{Account, Position, HistoricalTransaction};
use crate::overseer::traits::OverseenAccount;
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
}

#[wasm_bindgen]
pub struct JSPosition {
    #[wasm_bindgen(getter_with_clone)]
    pub vendor: String,
    #[wasm_bindgen(getter_with_clone)]
    pub security_id: String,
    #[wasm_bindgen(getter_with_clone)]
    pub security_name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub security_name_subtext: String,
    pub total_value: f32,
    pub total_cost: f32,
    pub current_price: f32,
    pub ppl: f32,
    pub ppl_as_perc: f32,
    pub quantity: f32
}

#[wasm_bindgen]
pub struct JSAccount {
    #[wasm_bindgen(getter_with_clone)]
    pub vendor: String,
    pub blocked: f32,
    pub free: f32,
    pub total_funds: f32,
    pub invested: f32,
    pub ppl: f32,
    pub total: f32
}

#[wasm_bindgen]
pub struct JSHistoricalTransaction {
    #[wasm_bindgen(getter_with_clone)]
    pub security_id: String,
    #[wasm_bindgen(getter_with_clone)]
    pub security_name: Option<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub security_name_subtext: Option<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub date: String,
    pub unit_cost: f32,
    pub quantity: f32,
    pub cost: f32,
    #[wasm_bindgen(getter_with_clone)]
    pub transaction_type: String,
}

#[wasm_bindgen]
pub fn test(positon_id: ) -> String{
    Position.get_security_id()
}


