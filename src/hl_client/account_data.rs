use super::HL;
 
use scraper::Selector;

pub struct Cash {
    pub blocked: f32,
    pub free: f32,
    pub invested: f32,
    pub ppl: f32,
    pub result: f32,
    pub total: f32,
}

impl HL {

    pub async fn fetch_account_cash(&self) {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 
        
        let table_css_selector = r#"table[id="portfolio"]"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();
        let inner_value = parsed_html
            .select(&html_selector);

        for value in inner_value {
            println!("Vakldsnoinasfd{:?}", &value.text());
        }


        
        let table_css_selector = r#"input[id="holdings-table"] tfoot tr"#;
        let table_cell_selector = [
            r#"td[id="stock_total"]"#,
            r#"td[id="costgbp_total"]"#,
            r#"td[id="gain_total"] div span[id="gaingbp_total"]"#,
            r#"td[id="gain_total"] div span[id="gainpc_total"]"#,
        ].map(String::from).to_vec();

        let account_totals = table_cell_selector
            .into_iter()
            .map(
                |selector| {
                    let html_selector = Selector::parse(&selector)
                        .unwrap();
                    let inner_value = parsed_html
                        .select(&html_selector)
                        .next()
                        .unwrap()
                        .inner_html();
                    println!("{}", &inner_value);
                }
            );
        

    }

}
