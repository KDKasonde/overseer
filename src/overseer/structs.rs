use std::ops::AddAssign;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use super::traits::ReadableSecurity;
use super::enums::{Asset, TransactionType};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Portfolio {
    pub accounts: Vec<Account>,
}

impl Portfolio {
    pub fn new() -> Self {
        Portfolio::default()
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub source: String,
    pub blocked: Decimal,
    pub free: Decimal,
    pub total_funds: Decimal,
    pub invested: Decimal,
    pub ppl: Decimal,
    pub total: Decimal,
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

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub source: String,
    pub asset: Asset,
    pub total_value: Decimal,
    pub total_cost: Decimal,
    pub current_price: Decimal,
    pub ppl: Decimal,
    pub ppl_as_perc: Decimal,
    pub quantity: Decimal,
}

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoricalTransaction {
    pub asset: Asset,
    pub date: DateTime<Utc>,
    pub unit_price: Decimal,
    pub quantity: Decimal,
    pub total_value: Decimal,
    pub transaction_type: TransactionType,
}

impl ReadableSecurity for Position {
    fn get_asset(&self) -> Asset {
        self.asset.clone()
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}