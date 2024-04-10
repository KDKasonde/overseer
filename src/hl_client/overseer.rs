use async_trait::async_trait;

use super::HL;
use super::portfolio_data::OpenPosition;

use crate::overseer::traits::{OverseenAccount, OverseerError, ReadableSecurity};
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

    async fn get_cash(&self) -> Result<Account, OverseerError> {
        let native_account = match self.fetch_all_account_cash().await {
            Ok(account) => {
                account
            }, 
            Err(e) => {
                return e
            }
        };

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
        
        let native_historical_transactions = self.fetch_historical_transaction(position).await;
        native_historical_transactions                 
            .iter()
            .map({
                |native_historical_transaction| {
                    HistoricalTransaction {
                        security_id: native_historical_transaction.security_id.to_owned(),
                        security_name: Some(native_historical_transaction.security_name.to_owned()),
                        security_name_subtext: Some(native_historical_transaction.security_name_subtext.to_owned()),
                        date: native_historical_transaction.date.to_owned(),
                        unit_cost: native_historical_transaction.unit_cost,
                        quantity: native_historical_transaction.quantity,
                        cost: native_historical_transaction.cost,
                        transaction_type: native_historical_transaction.transaction_type.to_owned()
                    }
                }
            })
        .collect::<Vec<HistoricalTransaction>>()

    }
}
