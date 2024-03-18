use std::collections::HashMap;

use super::HL;
use scraper::Html;
 
use scraper::Selector;

pub struct Cash {
    pub blocked: Option<f32>,
    pub free: Option<f32>,
    pub total_funds: f32,
    pub invested: f32,
    pub ppl: f32,
    pub total: f32,
}

impl HL {

    pub async fn fetch_account_cash(&self) -> Option<Cash> {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 
        
        let table_css_selector = r#"table[id="portfolio"] tbody tr"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();

        let mut mapping:HashMap<String, f32> = HashMap::with_capacity(6);
        
        let available_cash_selector = Selector::parse(r#"td a[title="Available"]"#)
            .unwrap();
        let available_cash = parsed_html
            .select(&available_cash_selector)
            .next()
            .unwrap()
            .inner_html()
            .trim()
            .replace("£", "")
            .replace(",","")
            .parse::<f32>()
            .ok();

        for row in parsed_html.select(&html_selector).into_iter() {
            let raw_page = self.navigate_to_link_(r#"td a[title="Stock summary"]"# ,&row.to_owned()).await;
            let account_page = if let Some(page) = raw_page  {
                page
            } else {
                continue
            };
            let key_value_list  = parse_account_information(account_page);
            let _: Vec<_> = key_value_list
                .into_iter()
                .map(|key_value| 
                     {
                         let key = &key_value.0;
                         let value = key_value.1;

                         if let Some(new_value) = value {
                             mapping.entry(key.to_string()).and_modify(|cur_value| *cur_value+= new_value).or_insert(new_value);
                         }            
                    }
                )
                .collect();
        }
        println!("{:?}", &mapping);
        return Some(
            Cash {
                blocked: Some(
                             *mapping.get("account_total")? - *mapping.get("total_stock_value")? - available_cash?
                             ),
                free: available_cash,
                total_funds: *mapping.get("account_total")?,
                invested: *mapping.get("total_invested")?,
                ppl: *mapping.get("ppl")?,
                total:* mapping.get("account_total")?
            }
        )

    }

}

fn parse_account_information(parsed_account_page: Html) -> Vec<(String,Option<f32>)> {
    let table_cell_selectors = [
        ("account_total", r#"td[id="account_total_header"]"#),
        ("total_stock_value", r#"td[id="stock_total"]"#),
        ("total_invested", r#"td[id="costgbp_total"]"#),
        ("ppl",r#"span[id="gaingbp_total"]"#),
        ("ppl_as_perc", r#"span[id="gainpc_total"]"#),
    ].to_vec();

    let account_kpis: Vec<(String, Option<f32>)> = table_cell_selectors
        .iter()
        .map(
            |(label, selector)| -> (String, Option<f32>) {
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
                (label.to_string(), inner_value.parse::<f32>().ok())
            }
        )
        .collect();
    account_kpis

}

