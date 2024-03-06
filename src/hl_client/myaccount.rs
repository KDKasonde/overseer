use super::HL;

impl HL {

    pub async fn check_account(&self) {
        let path = "https://online.hl.co.uk/my-accounts";

        let html = &self.client
            .get(path)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", html);
    }


}
