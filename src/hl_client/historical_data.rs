use crate::overseer::traits::ReadableSecurity;

use super::HL;
use super::utils::ScrapedValue;

use serde::Deserialize;
use scraper::{ElementRef, Selector};

/// Struct holding data about a transaction 
/// that took place in the past.
#[derive(Debug, Deserialize)]
pub struct HistoricalOrder {
    /// Id of the security the transaction took place on.
    pub security_id: String,
    /// The name of the security.
    pub security_name: String,
    /// The sub text e.g. class of shares.
    pub security_name_subtext: String,
    /// Date the transaction was executed.
    pub date: String,
    /// The cost of the transaction.
    pub unit_cost: f32,
    /// Multiplier for the unit_cost.
    pub quantity: f32,
    /// Total cost.
    pub cost: f32,
    /// The type of transaction that took place.
    pub transaction_type: String,
}

impl HL {

    /// Fetches historical transactions for a given security id,
    /// parsing through the resulting HTML table returned by the 
    /// request. Using each row as a historical transaction.
    pub async fn fetch_historical_transaction(&self, security_id: String, security_name: String, security_name_subtext: String) -> Vec<HistoricalOrder> {
        let security_url = format!("{}/my-accounts/security_movements/sedol/{}", self.base_url, security_id); 
        
        let parsed_html = self.fetch_url(security_url).await.unwrap();

        let transactions_selector = Selector::parse(r#"div[id="movements-table-container"] table tbody tr"#)
            .unwrap();

        let mut historical_transactions = Vec::new();

        for row in parsed_html.select(&transactions_selector).into_iter() {
            
            let historical_transaction = parse_transaction_information(&row);
            let transaction = HistoricalOrder {
                security_id: security_id.clone(),
                security_name: security_name.clone(),
                security_name_subtext: security_name_subtext.clone(),
                date: <Option<ScrapedValue> as Clone>::clone(&historical_transaction[0].1).unwrap().try_into().unwrap(),
                unit_cost: <Option<ScrapedValue> as Clone>::clone(&historical_transaction[1].1).unwrap().try_into().unwrap(),
                quantity: <Option<ScrapedValue> as Clone>::clone(&historical_transaction[2].1).unwrap().try_into().unwrap(),
                cost: <Option<ScrapedValue> as Clone>::clone(&historical_transaction[3].1).unwrap().try_into().unwrap(),  
                transaction_type: <Option<ScrapedValue> as Clone>::clone(&historical_transaction[4].1).unwrap().try_into().unwrap(),
            };
            historical_transactions.push(transaction);
        }
        historical_transactions 
    } 

    /// Fetches all historical transactions on the account.
    pub async fn fetch_all_historical_transactions(&self, securities: Vec<Box<dyn ReadableSecurity>>) -> Vec<HistoricalOrder> {
        let mut historical_transactions: Vec<HistoricalOrder> = Vec::new();

        for position in securities { 
            let security_id = position.get_security_id();
            let security_name = position.get_security_name().unwrap_or_else(|| String::new());
            let security_name_subtext = position.get_security_name_subtext().unwrap_or_else(|| String::new());

            let mut transactions: Vec<HistoricalOrder> = self.fetch_historical_transaction(security_id, security_name, security_name_subtext).await;
            historical_transactions.append(&mut transactions);
        }
        historical_transactions



    }

}


fn parse_transaction_information(parsed_account_page: &ElementRef) -> Vec<(String, Option<ScrapedValue>)> {
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
