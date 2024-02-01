
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

    pub async fn fetch_account_cash(&self) -> Result<Cash, reqwest::Error> {
        
        let client = &self.client;
        let target_url = format!("{}equity/account/cash", self.base_url );

        let res = client
            .get(target_url)
            .send()
            .await;
        
        let output = match res {
            Ok(response) => { 
                response
                    .json::<Cash>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        return output
