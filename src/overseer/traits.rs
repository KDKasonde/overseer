use async_trait::async_trait;


use super::structs::{Account, HistoricalTransaction, Position};
use super::errors::OverseerError;
use std::future::Future;


#[async_trait(?Send)]
pub trait OverseenAccount {
    async fn get_cash(&self) -> Result<Vec<Result<Account,OverseerError>>,OverseerError>;
    async fn get_asset_summary (&self) -> Vec<Position>;
    async fn get_historical_transactions (&self, position: Box<dyn ReadableSecurity>) -> Vec<HistoricalTransaction>;    
    async fn get_all_historical_transactions(&self) -> Vec<HistoricalTransaction> {
        let all_positions = self.get_asset_summary().await;
        let mut history = Vec::new();

        for position in all_positions {
            let mut transactions = self.get_historical_transactions(Box::new(position)).await;
            history.append(&mut transactions);
        }
        history
    }
    async fn login(&self, username: Option<String>, date_of_birth: Option<String>, password: Option<String>, secure_number: Option<String>) {
        panic!("login is not required for this account!")
    }
    async fn logout(&self) {
        panic!("logout is not required for this account!")
    }
}

pub trait LoginRequired {
    fn login(&self) -> impl Future<Output = ()>;
    fn logout(&self) -> impl Future<Output = ()>;
}

pub trait ReadableSecurity {
    fn get_security_id(&self) -> String;
    fn get_security_name(&self) -> Option<String> {
        None
    }
    fn get_security_name_subtext(&self) -> Option<String> {
        None
    }
    fn get_vendor(&self) -> String;
}
 
