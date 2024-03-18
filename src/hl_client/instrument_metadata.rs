use super::HL;

use serde::{de::Error, Deserialize};
use scraper::Selector;


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

     
    pub async fn fetch_portfolio_position(&self) {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 
        
        let table_css_selector = r#"table[id="portfolio"] tbody tr"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();

        for row in parsed_html.select(&html_selector).into_iter() {
        }
    } 

}


