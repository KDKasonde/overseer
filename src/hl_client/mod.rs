use reqwest::{
    Client,
    header,
};
use scraper::Html;

pub struct HL {
    base_url: String,
    client: Client, 
}

impl HL {
    pub fn new(base_url: &str, api_key: &str) -> HL {
        let mut headers = header::HeaderMap::new();

        headers.insert("User-Agent", header::HeaderValue::from_static("OverSeer"));

        let mut auth_value = header::HeaderValue::from_str(api_key).unwrap();
        
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
                       
        let client_builder = Client::builder()
            .default_headers(headers)
            .build();
        
        let output_client = match client_builder {
            Ok(client) => client,
            Err(_error) => panic!("Error creating client instance!")
        };

        HL {
            client: output_client,
            base_url: base_url.to_string()
        }
    }

}

pub async fn login_step_one<'a>(base_url: &'a str, username: &str, date_of_birth: &str) -> Option<String> {
    let path = format!("{}/login-step-one",base_url);
    let client = reqwest::Client::new();


    let res = client.post(path)
        .form(&[("username", username), ("date-of-birth",date_of_birth)])
        .send()
        .await;
        
    let output =res
                .unwrap()
                .text()
                .await;

    match output {
        Ok(html) => {
            let parsed_doc = Html::parse_document(&html);
            retrieve_hl_vt(&parsed_doc)
        },
        Err(error) => {
            println!("{error}");
            None
        }
    }
    
}

fn retrieve_hl_vt(parsed_doc: &Html) -> Option<String> {
    let selector = scraper::Selector::parse(r#"input[name="hl_vt"]"#).unwrap();

    let input = parsed_doc
        .select(&selector)    
        .next();
    if let Some(input_field) = input {
        input_field
            .value()
            .attr("value")
            .map(str::to_string)
    } else {
        None 
    }
}

