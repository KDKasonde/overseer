use wasm_bindgen::prelude::*;
use serde::Deserialize;

use crate::overseer::structs::{Account, Position, HistoricalTransaction};
use crate::overseer::traits::OverseenAccount;


pub enum OverSeen { 
     pub ApiAccount, 
     pub LoginAccount(Box<dyn OverseenAccount>)
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


