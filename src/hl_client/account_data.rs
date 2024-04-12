use std::collections::HashMap;

use crate::overseer::errors::OverseerError;

use super::HL;
use scraper::Html;
 
use scraper::Selector;

pub struct Cash {
    pub blocked: f32,
    pub free: f32,
    pub total_funds: f32,
    pub invested: f32,
    pub ppl: f32,
    pub total: f32,
}

impl HL {

    pub async fn fetch_account_cash(&self, account_link: &String, free_funds: f32) -> Result<Cash, OverseerError> {

        let raw_page = self.fetch_url(account_link.to_string()).await;
        let account_page = if let Some(page) = raw_page  {
            page
        } else {
            return Err(OverseerError::FailedFetch { url: account_link.to_owned() })
        };
        
        let key_value_list  = parse_account_information(&account_page);

        let mut mapping:HashMap<String, f32> = HashMap::with_capacity(6);
        
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

        let account_total = if let Some(data) = mapping.get("account_total") {
            data
        } else {
            return Err(OverseerError::MissingData { dataField: "account_total".to_string()})
        };

        let total_stock_value = if let Some(data) = mapping.get("total_stock_value") {
            data
        } else {
            return Err(OverseerError::MissingData { dataField: "total_stock_value".to_string()})
        };

        let total_invested = if let Some(data) = mapping.get("total_invested") {
            data
        } else {
            return Err(OverseerError::MissingData { dataField: "total_invested".to_string()})
        };

        let ppl = if let Some(data) = mapping.get("ppl") {
            data
        } else {
            return Err(OverseerError::MissingData { dataField: "ppl".to_string()})
        };

        return Ok(
            Cash {
                blocked: account_total - total_stock_value - free_funds,
                free: free_funds,
                total_funds: account_total - total_stock_value,
                invested: *total_invested,
                ppl: *ppl,
                total: *account_total
            }
        )

    }

    pub async fn fetch_all_account_cash(&self) -> Result<Vec<Result<Cash, OverseerError>>, OverseerError> {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 
        
        let table_css_selector = r#"table[id="portfolio"] tbody tr"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();

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

        let available_cash = match available_cash {
            Some(cash) => { cash },
            _ =>  {
                return Err(OverseerError::MissingData { dataField: "available_cash".to_string()})
            }
        };


        let mut all_accounts = Vec::new();
        for row in parsed_html.select(&html_selector).into_iter() {
            let account = if let Some(link) = get_link(r#"td a[title="Stock summary"]"# ,&row.to_owned()) {
                link 
            } else {
                continue 
            }; 

            let account = self.fetch_account_cash(&account, available_cash).await;
        
            all_accounts.push(account);    
        }

        return Ok(
            all_accounts
        )

    }

}

fn parse_account_information(parsed_account_page: &Html) -> Vec<(String,Option<f32>)> {
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

fn get_link<'a>(css_selector: &str, raw_html: &scraper::ElementRef<'a>) -> Option<String> {
    let html_selector = Selector::parse(css_selector).ok()?;
    let link = raw_html
        .select(&html_selector)
        .next()?;
    Some(link.attr("href")?.to_string())
}


