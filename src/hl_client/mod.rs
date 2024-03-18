mod instrument_metadata;
mod account_data;
mod login;

use std::sync::Arc;
use reqwest::{
    cookie::Jar, header, Client, Url
};
use scraper::{Html, Selector};

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
}

