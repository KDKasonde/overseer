use wasm_bindgen::prelude::*;
use crate::{
    hl_client::HL,
    trading212_api::Trading212,
};

pub enum Vendor {
    HL(HL),
    Trading212(Trading212)
}


