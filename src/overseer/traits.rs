use async_trait::async_trait;


use super::structs::{Account, HistoricalTransaction, Position};
use super::errors::OverseerError;

/// Interface to allow all types of vendor accounts to be summarised as 
/// one and enable more target devices to access their implementations
#[async_trait(?Send)]
pub trait OverseenAccount {
    /// Retrieve the account information and cash balance.
    async fn get_cash(&self) -> Vec<Result<Account,OverseerError>>;
    /// Retrieve the assets currently held within the account.
    async fn get_asset_summary (&self) -> Vec<Position>;
    /// Retrieve the historical transactions of the position that has been passed into 
    /// the function.
    async fn get_historical_transactions (&self, position: Box<dyn ReadableSecurity>) -> Vec<HistoricalTransaction>;
    /// Retrieve all historical transactions on the account.
    async fn get_all_historical_transactions(&self) -> Vec<HistoricalTransaction> {
        let all_positions = self.get_asset_summary().await;
        let mut history = Vec::new();

        for position in all_positions {
            let mut transactions = self.get_historical_transactions(Box::new(position)).await;
            history.append(&mut transactions);
        }
        history
    }
    /// Login to account if required.
    async fn login(&self, username: Option<String>, date_of_birth: Option<String>, password: Option<String>, secure_number: Option<String>) {
        panic!("login is not required for this account!")
    }
    /// End a login session safely.
    async fn logout(&self) {
        panic!("logout is not required for this account!")
    }
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
 
