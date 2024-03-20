use super::HL;
use crate::hl_client::utils::ScrapedValue;

use serde::Deserialize;
use scraper::{ElementRef, Selector};




#[derive(Debug, Deserialize)]
pub struct OpenPosition {
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

#[derive(Debug, Deserialize)]
pub struct AllOpenPositions {
    pub positions: Option<Vec<Option<OpenPosition>>>,
}

impl HL {

     
    pub async fn fetch_portfolio_position(&self) -> Vec<OpenPosition> {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 

        let table_css_selector = r#"table[id="portfolio"] tbody tr"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();

        let mut all_open_positions: Vec<OpenPosition> = Vec::new();

        for row in parsed_html.select(&html_selector).into_iter() {

            let account_page = self.navigate_to_link_(r#"td a[title="Stock summary"]"# ,&row.to_owned()).await.unwrap();

            let table_css_selector = r#"table[id="holdings-table"] tbody tr"#;
            let html_selector = Selector::parse(&table_css_selector).unwrap();

            let holdings_table = account_page.select(&html_selector); 

            for row in holdings_table.into_iter() {
                let instrument_data = parse_account_information(&row);

                all_open_positions.push(
                    OpenPosition{
                        security_id: <Option<ScrapedValue> as Clone>::clone(&instrument_data[0].1).unwrap().try_into().unwrap(),
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

}


fn parse_account_information(parsed_account_page: &ElementRef) -> Vec<(String,Option<ScrapedValue>)> {
    let table_cell_selectors = [
        ("security_id", r#"td:nth-child(1) a[title="View trading history"]"#),
        ("security_name", r#"td:nth-child(2) a[title="View trading history"] span"#),
        ("security_name_subtext", r#"td:nth-child(2) span.text-mini"#),
        ("units_held", r#"td:nth-child(3) span"#),
        ("price", r#"td:nth-child(4) span"#),
        ("value", r#"td:nth-child(5) span span"#),
        ("cost", r#"td:nth-child(6) span"#),
        ("yeild", r#"td:nth-child(7) span"#),
        ("daily_delta", r#"td.day-change span"#),
        ("daily_delta_as_perc", r#"td.day-change div:nth-child(2) span"#),
        ("daily_ppl", r#"td.daily-gain-loss span"#),
        ("daily_ppl_as_perc", r#"td.daily-gain-loss  div:nth-child(2) span"#),
        ("ppl", r#"td:nth-child(10) span:nth-child(1)"#),
        ("ppl_as_perc", r#"td:nth-child(10) div:nth-child(2) span"#),
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
