use async_trait::async_trait;

use super::HL;
use super::portfolio_data::OpenPosition;

use crate::overseer::traits::{OverseenAccount, ReadableSecurity};
use crate::overseer::errors::OverseerError;
use crate::overseer::structs::{Account, HistoricalTransaction, Position}; 

impl ReadableSecurity for OpenPosition {
    fn get_security_id(&self) -> String {
        self.security_id.to_owned()
    }

    fn get_security_name(&self) -> Option<String> {
       Some(self.security_name.to_owned())
    }

    fn get_security_name_subtext(&self) -> Option<String> {
        Some(self.security_name_subtext.to_owned())
    }
     
    fn get_vendor(&self) -> String {
       "Hargeaves Lansdown".to_string()
    }
}

#[async_trait(?Send)]
impl OverseenAccount for HL {

    async fn get_cash(&self) -> Vec<Result<Account,OverseerError>> {
        let native_accounts = match self.fetch_all_account_cash().await {
            Ok(account) => {
                account
            }, 
            Err(e) => {
                panic!("No accounts could be retrieved")
            }
        };
        let overseer_accounts = native_accounts 
            .into_iter()
            .map(
                |account| {
                    match account {
                        Ok(native_account) => {
                            Ok(
                                Account{
                                    vendor: "Hargeaves Lansdown".to_string(),
                                    blocked: native_account.blocked,
                                    free: native_account.blocked,
                                    total_funds: native_account.total_funds,
                                    invested: native_account.invested,
                                    ppl: native_account.ppl,
                                    total: native_account.total
                                }
                              )
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
            )
            .collect::<Vec<Result<Account,OverseerError>>>();

        overseer_accounts

    }

    async fn get_asset_summary(&self) -> Vec<Position> {
        let native_summary = self.fetch_portfolio_position().await;

        native_summary
            .iter()
            .map({
                |native_position| {
                    Position {
                        vendor: "Hargeaves Lansdown".to_string(),
                        security_id: native_position.security_id.to_owned(),
                        security_name: native_position.security_name_subtext.to_owned(),
                        security_name_subtext: native_position.security_name_subtext.to_owned(),
                        total_value: native_position.total_value,
                        total_cost: native_position.total_cost,
                        current_price: native_position.current_price,
                        ppl: native_position.ppl,
                        ppl_as_perc: native_position.ppl_as_perc,
                        quantity: native_position.quantity
                    }
                }
            })
        .collect::<Vec<Position>>()
    }
    
    async fn get_historical_transactions(&self, position: Box<dyn ReadableSecurity>) -> Vec<HistoricalTransaction> {
        let security_id = position.get_security_id();
        let security_name = position.get_security_name().unwrap_or_else(|| String::new());
        let security_name_subtext = position.get_security_name_subtext().unwrap_or_else(|| String::new());

        let native_historical_transactions = self.fetch_historical_transaction(security_id, security_name, security_name_subtext).await;
        native_historical_transactions                 
            .iter()
            .map({
                |native_historical_transaction| {
                    HistoricalTransaction {
                        security_id: native_historical_transaction.security_id.to_owned(),
                        security_name: Some(native_historical_transaction.security_name.to_owned()),
                        security_name_subtext: Some(native_historical_transaction.security_name_subtext.to_owned()),
                        date: native_historical_transaction.date.to_owned(),
                        unit_value: native_historical_transaction.unit_value,
                        quantity: native_historical_transaction.quantity,
                        value: native_historical_transaction.value,
                        transaction_type: native_historical_transaction.transaction_type.to_owned()
                    }
                }
            })
        .collect::<Vec<HistoricalTransaction>>()

    }
     
    async fn login(&self, username: Option<String>, date_of_birth: Option<String>, password: Option<String>, secure_number: Option<String>) {
        let username = if let Some(user) = username {
            user 
        } else {
            panic!("Missing username")
        };
        let password = if let Some(pass) = password {
            pass
        } else {
            panic!("Missing password")
        };
        let secure_number = if let Some(number) = secure_number {
            number
        } else {
            panic!("Missing secure number")
        };
        let date_of_birth = if let Some(date) = date_of_birth {
            date
        } else {
            panic!("Missing secure number")
        };

        self.login(username, date_of_birth, password, secure_number).await;
    }

    async fn logout(&self) {
        self.logout().await
    }
}
