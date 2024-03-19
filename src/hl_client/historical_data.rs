use crate::hl_client::Account;

use super::HL;

use serde::Deserialize;
use scraper::{selectable::Selectable, ElementRef, Selector};


#[derive(Debug, Clone)]
enum ScrapedValue {
    Str(String),
    Float(f32)
}


#[derive(Debug, Deserialize)]
pub struct HistoricalTransaction {
    pub security_name: String,
    pub security_name_subtext: String,
    pub unit_cost: f32,
    pub quantity: f32,
    pub cost: f32,
    pub reference: f32,
    pub transaction_type: f32,
}

impl HL {

    pub async fn fetch_transactions(&self, account_list: Option<Vec<Account>>) -> Vec<HistoricalTransaction> {

        let all_accounts = if let Some(accounts) = account_list {
            accounts 
        } else {
            self.fetch_accounts().await
        };

        for row in parsed_html.select(&html_selector).into_iter() {

            let account_page = self.navigate_to_link_(r#"td a[title="Stock summary"]"# ,&row.to_owned()).await.unwrap();

            let table_css_selector = r#"table[id="holdings-table"] tbody tr"#;
            let html_selector = Selector::parse(&table_css_selector).unwrap();

            let holdings_table = account_page.select(&html_selector); 

            for row in holdings_table.into_iter() {
                let instrument_data = parse_account_information(&row);

                all_open_positions.push(
                    OpenPosition{
                        security_name: <Option<ScrapedValue> as Clone>::clone(&instrument_data[1].1).unwrap().try_into().unwrap(),
                        security_name_subtext: <Option<ScrapedValue> as Clone>::clone(&instrument_data[2].1).unwrap().try_into().unwrap(),
                        quantity: <Option<ScrapedValue> as Clone>::clone(&instrument_data[3].1).unwrap().try_into().unwrap(),
                        current_price: <Option<ScrapedValue> as Clone>::clone(&instrument_data[4].1).unwrap().try_into().unwrap(),
                        total_value : <Option<ScrapedValue> as Clone>::clone(&instrument_data[5].1).unwrap().try_into().unwrap(),
                        total_cost: <Option<ScrapedValue> as Clone>::clone(&instrument_data[6].1).unwrap().try_into().unwrap(),
                        ppl: <Option<ScrapedValue> as Clone>::clone(&instrument_data[12].1).unwrap().try_into().unwrap(),
                        ppl_as_perc: <Option<ScrapedValue> as Clone>::clone(&instrument_data[13].1).unwrap().try_into().unwrap(),
                    }
                    )

            }
        }
        println!("{:?}", &all_open_positions);
        all_open_positions
    } 

    pub async fn fetch_transactions_in_account(&self, account_link: &String) -> Account {
        let parsed_html = self.fetch_url(account_link.to_string()).await.unwrap();

        let instrument_link_selector = r#"table[id="holdings-table"]"#;

        for row in parsed_html.select(&instrument_link_selector).into_iter() {
            let instrument_page_raw = self.navigate_to_link_(r#"a[title="View trading history"]"#, &row.to_owned()).await;

            let insturment_page = if let Some(page) = instrument_page_raw {
                page
            } else {
                continue 
            };

            let transactions_selector = Selector::parse(r#"div[id="movements-table-container"] table tbody tr"#)
                .unwrap();
            let transactions_table = insturment_page
                .select(&transactions_selector)
                .into_iter()
                .map(
                    |row| {
                        let instrument_transactions = parse_transaction_information(&row);
                    }
                    )
                .collect();



        }
    }
}


fn parse_transaction_information(parsed_account_page: &ElementRef) -> Vec<HistoricalTransaction> {
    let table_cell_selectors = [
        ("Date", r#"td:nth-child(1)"#),
        ("Type", r#"td:nth-child(2)"#),
        ("unit_cost", r#"td:nth-child(4)"#),
        ("quantity", r#"td:nth-child(5)"#),
        ("cost", r#"td:nth-child(6)"#),
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
