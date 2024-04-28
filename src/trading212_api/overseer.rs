use async_trait::async_trait;
use super::Trading212;
use super::portfolio_data::OpenPosition;

use crate::overseer::traits::{OverseenAccount, ReadableSecurity};
use crate::overseer::errors::OverseerError;
use crate::overseer::structs::{Account, HistoricalTransaction, Position}; 

impl ReadableSecurity for OpenPosition {
    fn get_security_id(&self) -> String {
        self.ticker.to_owned()
    }

    fn get_vendor(&self) -> String {
        "Trading 212".to_string()    
    }
}

#[async_trait(?Send)]
impl OverseenAccount for Trading212 {

    async fn get_cash(&self) -> Vec<Result<Account,OverseerError>> {
        let native_account = self.fetch_account_cash().await;

        let overseer_accounts = native_account 
            .into_iter()
            .map(
                |native_account| {
                    let blocked = native_account.blocked.unwrap_or(0.);
                    Ok(
                        Account{
                            vendor: "Hargeaves Lansdown".to_string(),
                            blocked,
                            free: native_account.free,
                            total_funds: native_account.free + native_account.pie_cash + blocked,
                            invested: native_account.invested,
                            ppl: native_account.ppl,
                            total: native_account.total
                        }
                    )
                }
            )
            .collect::<Vec<Result<Account,OverseerError>>>();

        overseer_accounts

    }

    async fn get_asset_summary(&self) -> Vec<Position> {
        let native_summary = self.fetch_portfolio_positions().await;

        native_summary
            .iter()
            .map({
                |native_position| {
                    Position {
                        vendor: "Trading 212".to_string(),
                        security_id: native_position.ticker.to_owned(),
                        security_name: "N/A".to_string(),
                        security_name_subtext: "N/A".to_string(),
                        total_value: native_position.quantity * native_position.current_price,
                        total_cost:native_position.quantity * native_position.average_price,
                        current_price: native_position.current_price,
                        ppl: native_position.ppl,
                        ppl_as_perc: 
                            (native_position.current_price - native_position.average_price)/
                            native_position.average_price,
                            quantity: native_position.quantity
                    }
                }
            })
        .collect::<Vec<Position>>()
    }
    
    async fn get_historical_transactions(&self, position: Box<dyn  ReadableSecurity>) -> Vec<HistoricalTransaction> {
        let native_historical_transactions = self.fetch_historical_orders(None, &position.get_security_id(), None)
            .await;
        native_historical_transactions
            .orders
            .iter()
            .map({
                |native_historical_transaction| {
                    HistoricalTransaction {
                        security_id: native_historical_transaction.ticker.to_owned(),
                        security_name: None,
                        security_name_subtext: None,
                        date: native_historical_transaction.date_executed.to_owned(),
                        unit_value: native_historical_transaction.limit_price,
                        quantity: native_historical_transaction.filled_quantity,
                        value: native_historical_transaction.filled_value,
                        transaction_type: native_historical_transaction.item_type.to_owned()
                    }
                }
            })
        .collect::<Vec<HistoricalTransaction>>()
    }
 
}
