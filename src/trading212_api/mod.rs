mod portfolio_data;
mod historical_data;
mod pie_data;
mod instrument_metadata;
mod account_data;

use reqwest::{
    Client,
    header,
};

pub struct Trading212 {
    
    client: Client,
    base_url: String,
    
}

impl Trading212 {
    
    pub fn new(base_url: &str, api_key: &str) -> Trading212 {
        let mut headers = header::HeaderMap::new();

        headers.insert("User-Agent", header::HeaderValue::from_static("OverSeer"));

        let mut auth_value = header::HeaderValue::from_str(api_key).unwrap();
        
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
                       
        let client_builder = Client::builder()
            .default_headers(headers)
            .build();
        
        let client = match client_builder {
            Ok(client) => client,
            Err(_error) => panic!("Error creating client instance!")
        };

        Trading212 {
            client: client,
            base_url: base_url.to_string()
        }
    }
}
