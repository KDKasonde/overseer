use super::HL;

use serde::{de::Error, Deserialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    pub average_price: f32,
    pub current_price: f32,
    pub frontend: String,
    pub fx_ppl: Option<f32>,
    pub initial_fill_date: String,
    pub max_buy: f32,
    pub max_sell: f32,
    pub pie_quantity: f32,
    pub ppl: f32,
    pub quantity: f32,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOpenPositions {
    pub positions: Option<Vec<Option<OpenPosition>>>,
}

impl HL {

     

}


