use std::ops::AddAssign;

#[cfg(feature="wasm")]
use wasm_bindgen::prelude::*;
use super::traits::ReadableSecurity;
use serde::{Serialize, Deserialize};

#[cfg_attr(feature="wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub vendor: String,
    pub blocked: f32,
    pub free: f32,
    pub total_funds: f32,
    pub invested: f32,
    pub ppl: f32,
    pub total: f32
}

impl AddAssign<&Account> for Account {
    fn add_assign(&mut self, rhs: &Account) {
        self.blocked += rhs.blocked;
        self.free += rhs.free;
        self.total_funds += rhs.total_funds;
        self.total += rhs.total;
        self.invested += rhs.invested;
        self.ppl += rhs.ppl; 
    }
}

#[cfg_attr(feature="wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize)]
pub struct Position {
    pub vendor: String,
    pub security_id: String,
    pub security_name: String,
    pub security_name_subtext: String,
    pub total_value: f32,
    pub total_cost: f32,
    pub current_price: f32,
    pub ppl: f32,
    pub ppl_as_perc: f32,
    pub quantity: f32
}

#[cfg_attr(feature="wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize)]
pub struct HistoricalTransaction {
    pub security_id: String,
    pub security_name: Option<String>,
    pub security_name_subtext: Option<String>,
    pub date: String,
    pub unit_value: f32,
    pub quantity: f32,
    pub value: f32,
    pub transaction_type: String,
}

#[cfg_attr(feature="wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize)]
pub struct HistoricalDividend {
    pub security_id: String,
    pub security_name: Option<String>,
    pub security_name_subtext: Option<String>,
    pub date: String,
    pub unit_value: f32,
    pub quantity: f32,
    pub value: f32,
}

impl ReadableSecurity for Position {
    fn get_security_id(&self) -> String {
        self.security_id.to_owned()
    }

    fn get_security_name(&self) -> Option<String> {
        Some(self.security_name.to_owned())
    }

    fn get_vendor(&self) -> String {
        self.vendor.to_owned()
    }
}

