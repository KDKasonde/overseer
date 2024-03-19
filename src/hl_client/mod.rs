mod portfolio_data;
mod historical_data;
mod account_data;
mod login;

use std::sync::Arc;
use reqwest::{
    cookie::Jar, header, Client, Url
};
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

use self::account_data::Cash;

pub struct Account {
    account_id: String,
    account_link: String,
    account_name: String,
    account_details: Cash
}

pub struct HL {
    base_url: String,
    client: Client,
}

impl HL {
    pub fn new(base_url: &str) -> HL {
        let mut headers = header::HeaderMap::new();

        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("overseer"));
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
        let cookie = "jsCheck=yes; path=/";
        let jar = Arc::new(Jar::default());

        jar.add_cookie_str(cookie,&base_url.parse::<Url>().unwrap());
        let client_builder = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .cookie_provider(jar.clone())
            .redirect(reqwest::redirect::Policy::none())
            .build();
        
        let output_client = match client_builder {
            Ok(client) => client,
            Err(_error) => panic!("Error creating client instance!")
        };

        HL {
            client: output_client,
            base_url: base_url.to_string(),
        }
    }
    
    async fn fetch_url(&self, url: String) -> Option<Html> {
        let client = &self.client;

        let html_text = client
            .get(&url)
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

        Some(Html::parse_document(&html_text))

    }

    async fn navigate_to_link_<'a>(&self, css_selector: &str, raw_html: &scraper::ElementRef<'a>) -> Option<Html> {
        let html_selector = Selector::parse(css_selector).ok()?;
        let link = raw_html
            .select(&html_selector)
            .next()?;

        let parsed_html = self.fetch_url(link.attr("href")?.to_string()).await.unwrap();
        Some(parsed_html)

    }

    async fn fetch_accounts(&self) -> Vec<Account> {
        let overview_url = "https://online.hl.co.uk/my-accounts/portfolio_overview"; 
        let parsed_html = self.fetch_url(overview_url.to_string()).await.unwrap(); 
        
        let table_css_selector = r#"table[id="portfolio"] tbody tr"#;
        let html_selector = Selector::parse(&table_css_selector)
            .unwrap();
        
        let mut accounts = Vec::new();

        for row in parsed_html.select(&html_selector).into_iter() {
            
            let (account_id, account_link, account_name, free_funds) = parse_account_information(row);
            let account_details  = self.fetch_account_cash(&account_link, free_funds).await.unwrap();

            let account = Account{
                account_id,
                account_link,
                account_name,
                account_details
            };

            accounts.push(account);
        }

        accounts
    }
}

fn parse_account_information(row: ElementRef) -> (String, String, String, f32) {
    let account_link_selector = Selector::parse(r#"a[title="Stock summary"]"#)
        .unwrap();
    let free_funds_selector = Selector::parse(r#"td a[title="Available"]"#)
        .unwrap();

    let account_link = row
        .select(&account_link_selector)
        .next()
        .unwrap()
        .attr("href")
        .unwrap()
        .to_string();

    let account_id = account_link
        .split("/")
        .last()
        .unwrap()
        .to_string();

    let account_name = row
        .select(&account_link_selector)
        .next()
        .unwrap()
        .inner_html()
        .to_string();

    let free_funds = row
        .select(&free_funds_selector)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .replace("£", "")
        .replace(",","")
        .parse::<f32>()
        .ok()
        .unwrap();

    (account_id, account_link, account_name,free_funds)
}

