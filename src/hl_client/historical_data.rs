use super::{portfolio_data::OpenPosition, HL};

use serde::Deserialize;
use scraper::{ElementRef, Selector};


#[derive(Debug, Clone)]
enum ScrapedValue {
    Str(String),
    Float(f32)
}


#[derive(Debug, Deserialize)]
pub struct HistoricalTransaction {
    pub security_name: String,
    pub security_name_subtext: String,
    pub date: String,
    pub unit_cost: f32,
    pub quantity: f32,
    pub cost: f32,
    pub transaction_type: f32,
}

impl HL {

    pub async fn fetch_historical_transaction(&self, security: &OpenPosition) -> Vec<HistoricalTransaction> {
        let security_url = format!("{}/my-accounts/security_movements/sedol/{}", self.base_url, security.security_id); 
        
        let parsed_html = self.fetch_url(security_url).await.unwrap();

        let transactions_selector = Selector::parse(r#"div[id="movements-table-container"] table tbody tr"#)
            .unwrap();

        let transactions_table = parsed_html
            .select(&transactions_selector);

        let mut historical_transactions = Vec::new();

        for row in transactions_table.into_iter() {
            
            let historical_transaction = parse_transaction_information(&transactions_table);

            let transaction = HistoricalTransaction {
                security_name: security.security_name,
                security_name_subtext: security.security_name_subtext,
                date: <Option<ScrapedValue> as Clone>::clone(historical_transaction[0].1).unwrap().try_into().unwrap(),
                unit_cost: <Option<ScrapedValue> as Clone>::clone(historical_transaction[1].1).unwrap().try_into().unwrap(),
                quantity: <Option<ScrapedValue> as Clone>::clone(historical_transaction[2].1).unwrap().try_into().unwrap(),
                cost: <Option<ScrapedValue> as Clone>::clone(historical_transaction[3].1).unwrap().try_into().unwrap(),  
                transaction_type: <Option<ScrapedValue> as Clone>::clone(historical_transaction[4].1).unwrap().try_into().unwrap(),
            };
            historical_transactions.push(transaction);
        }
        historical_transactions 
    } 

    pub async fn fetch_all_transactions(&self, securities: Vec<OpenPosition>) -> Vec<HistoricalTransaction> {
        let historical_transactions = Vec::new();

        securities
            .iter()
            .map(
                |security| {
                    let transactions = self.fetch_historical_transaction(&security);
                    historical_transactions.extend(transactions.iter());
                }
             )
            .collect();
        historical_transactions
    }
}


fn parse_transaction_information(parsed_account_page: &ElementRef) -> Vec<ScrapedValue> {
    let table_cell_selectors = [
        ("date", r#"td:nth-child(1)"#),
        ("unit_cost", r#"td:nth-child(4)"#),
        ("quantity", r#"td:nth-child(5)"#),
        ("cost", r#"td:nth-child(6)"#),
        ("type", r#"td:nth-child(2)"#),
    ].to_vec();

    let account_kpis: Vec<(String, Option<ScrapedValue>)> = table_cell_selectors
        .iter()
        .map(
            |(label, selector)| -> (String, Option<ScrapedValue>) {
                let html_selector = Selector::parse(&selector)
                    .expect("Selector is not parsable");
                let inner_value =parsed_account_page 
                    .select(&html_selector)
                    .next()
                    .expect(&format!("HTML not parsed for selector {:?}", &selector))
                    .inner_html()
                    .trim()
                    .replace("£", "")
                    .replace(",","");
                (label.to_string(), inner_value.parse::<ScrapedValue>().ok())
            }
        )
        .collect();
    account_kpis

}
