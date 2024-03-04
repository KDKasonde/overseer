use std::sync::Arc;
use reqwest::{
    cookie::Jar, header, Client, Url
};
use scraper::Html;

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



pub async fn login_step_one(&self, username: &str, date_of_birth: &str) -> Option<String> {
    let path = format!("{}/my-accounts/login-step-one",self.base_url);
    let client = &self.client;
    println!("{}", &path);
    let res = client.get(&path)
        .send()
        .await;
        
    let output = res
                .unwrap()
                .text()
                .await
                .expect("Error retrieving the login step one page");

    let parsed_doc = Html::parse_document(&output);
    
    let hl_vt = retrieve_hl_vt(&parsed_doc)
        .expect("No hl_vt was found.");

    let _ = client.post(&path)
        .form(&[("hl_vt", hl_vt), ("username", username), ("date-of-birth",date_of_birth)])
        .send()
        .await
        .expect("Error authenticating username and date of birth.")
        .text()
        .await
        .unwrap();

    let path = format!("{}/my-accounts/login-step-two",self.base_url);
    
    let output = client.get(&path)
        .send()
        .await
        .expect("Error authenticating username and date of birth.")
        .text()
        .await;

    match output {
        Ok(html) => {
            let parsed_doc = Html::parse_document(&html);
            login_step_two(&parsed_doc);
            Some("String".into())
        },
        Err(error) => {
            println!("{error}");
            panic!("No html returned"); 
        }
    }
    
}
}
fn login_step_two(parsed_doc: &Html) {

    let label_text_vec = vec![r#"input[id="secure-number-1"]"#, r#"input[id="secure-number-2"]"#, r#"input[id="secure-number-3"]"#]
        .into_iter()
        .map(
            |selector|{
                println!("{}", selector);
                let label_selector = scraper::Selector::parse(selector)
                    .expect("unable to find security number labels!");
                let label = parsed_doc
                    .select(&label_selector)
                    .next()
                    .expect("No matching label for {:selector}")
                    .attr("title");
                label
            }
        )
        .collect::<Vec<_>>(); 
   
    for label_text in label_text_vec {
        println!("{:?}", label_text)
    } 

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

