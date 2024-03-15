mod myaccount;
mod account_data;

use std::sync::Arc;
use reqwest::{
    cookie::Jar, header, Client, Url
};
use scraper::Html;
use regex::Regex;

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
    
    pub async fn login(
        &self,
        username: String,
        date_of_birth: String,
        password: String,
        secure_numbers: String
    ){
    
        let path = format!("{}/my-accounts/login-step-one",self.base_url);
        let client = &self.client;

        let res = client.get(&path)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .expect("Error retrieving the login step one page");

        let parsed_doc = Html::parse_document(&res);

        let hl_vt = retrieve_hl_vt(&parsed_doc)
            .expect("No hl_vt was found.")
            .to_string();
        
        self.post_username_and_dob(hl_vt.clone(), username, date_of_birth).await;
        
        let path = format!("{}/my-accounts/login-step-two",self.base_url);

        let html = client.get(&path)
            .send()
            .await
            .expect("Error authenticating username and date of birth.")
            .text()
            .await
            .unwrap();
        

        let parsed_doc = Html::parse_document(&html);
        let required_secure_numbers = parse_secure_numbers(&parsed_doc);
        self.post_password_and_secure_number(hl_vt, secure_numbers, password, required_secure_numbers).await;

    }

    pub async fn logout(&self) {
    }

    pub async fn post_password_and_secure_number(
        &self, 
        hl_vt: String, 
        secure_numbers: String, 
        password: String, 
        required_secure_number: Vec<usize>
    ) {

        let mut  params = vec![("hl_vt".to_string(), hl_vt), ("online-password-verification".to_string(), password)];

        for (index, number) in required_secure_number.iter().enumerate() {
            let input_name = format!("secure-number[{}]", index+1).to_string();
            let secure_number = secure_numbers.chars().collect::<Vec<char>>()[*number].to_string();    
            params.push((input_name, secure_number));
        }
        params.push(("submit".to_string(), "Log in".to_string()));
        let path = format!("{}/my-accounts/login-step-two",self.base_url);
        let _ = &self.client
            .post(&path)
            .form(&params)
            .send()
            .await;
        
    }

    async fn post_username_and_dob(&self,hl_vt: String, username: String, date_of_birth: String) {
        
        let client = &self.client;
        let path = format!("{}/my-accounts/login-step-one",self.base_url);
        
        let _ = client.post(&path)
            .form(&[("hl_vt", hl_vt), ("username", username), ("date-of-birth",date_of_birth)])
            .send()
            .await
            .expect("Error authenticating username and date of birth.")
            .text()
            .await
            .unwrap();
    }
    
    pub async fn fetch_url(&self, url: String) -> Option<Html> {
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

}

fn parse_secure_numbers(parsed_doc: &Html) -> Vec<usize> {
    let re = Regex::new(r"[0-9]").unwrap();

    let label_text_vec = vec![r#"input[id="secure-number-1"]"#, r#"input[id="secure-number-2"]"#, r#"input[id="secure-number-3"]"#]
        .into_iter()
        .map(
            |selector|{
                let label_selector = scraper::Selector::parse(selector)
                    .expect("unable to find security number labels!");
                let label = parsed_doc
                    .select(&label_selector)
                    .next()
                    .expect("No matching label for {:selector}")
                    .attr("title")
                    .unwrap();
                re
                    .find(label)
                    .expect("Could not determine the security number required for step two login.")
                    .as_str()
                    .parse::<usize>()
                    .unwrap() -1

                    
            }
        )
        .collect::<Vec<usize>>(); 
    label_text_vec
 }

fn retrieve_hl_vt(parsed_doc: &Html) -> Option<&str> {
    let selector = scraper::Selector::parse(r#"input[name="hl_vt"]"#).unwrap();

    let input = parsed_doc
        .select(&selector)    
        .next();
    if let Some(input_field) = input {
        input_field
            .value()
            .attr("value")
    } else {
        None 
    }
}

