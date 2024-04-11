use wasm_bindgen::prelude::*;
use super::traits::ReadableSecurity;
use super::enums::Vendor;

#[wasm_bindgen]
pub struct OverseerVendor {
    pub vendor: Vendor,
}

#[wasm_bindgen]
pub struct Account {
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
pub struct Position {
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
pub struct HistoricalTransaction {
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

