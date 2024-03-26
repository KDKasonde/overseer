use std::future::Future;

use super::HL;
use super::portfolio_data::OpenPosition;

use crate::overseer::traits::{OverseenAccount, ReadableSecurity};
use crate::overseer::structs::{Account, HistoricalTransaction, Position}; 

impl ReadableSecurity for OpenPosition {
    fn get_security_id(&self) -> String {
        self.security_id.to_owned()
    }

    fn get_security_name(&self) -> String {
        self.security_name.to_owned()
    }
}

impl OverseenAccount for HL {

    fn get_cash(&self) -> impl Future<Output = Account> {
        let converted_account = async {
            let native_account = self.fetch_all_account_cash().await.unwrap();
            Account{
                vendor: "Hargeaves Lansdown".to_string(),
                blocked: native_account.blocked,
                free: native_account.blocked,
                total_funds: native_account.total_funds,
                invested: native_account.invested,
                ppl: native_account.ppl,
                total: native_account.total
            }

        };
        converted_account
    }

    fn get_asset_summary(&self ) ->impl Future<Output=Vec<Position>> {
        let converted_summary = async {
            let native_summary = self.fetch_portfolio_position().await;

            native_summary
                .iter()
                .map({
                    |native_position| {
                        Position {
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
        };

        converted_summary        
    }
    
    fn get_historical_transactions(&self, position: impl ReadableSecurity) -> impl Future<Output = Vec<HistoricalTransaction>> {
        let converted_transactions = async move {
            let native_historical_transactions = self.fetch_historical_transaction(&position).await;
            native_historical_transactions                 .iter()
                .map({
                    |native_historical_transaction| {
                        HistoricalTransaction {
                            security_name: native_historical_transaction.security_name.to_owned(),
                            security_name_subtext: native_historical_transaction.security_name_subtext.to_owned(),
                            date: native_historical_transaction.date.to_owned(),
                            unit_cost: native_historical_transaction.unit_cost,
                            quantity: native_historical_transaction.quantity,
                            cost: native_historical_transaction.cost,
                            transaction_type: native_historical_transaction.transaction_type.to_owned()
                        }
                    }
                })
            .collect::<Vec<HistoricalTransaction>>()
        }; 
        converted_transactions

    }

    fn get_all_historical_transactions(&self) -> impl Future<Output = Vec<HistoricalTransaction>> {
        let converted_transactions = async move { 
            let all_positions = self.get_asset_summary().await;
            let native_historical_transactions = self.fetch_all_historical_transactions(all_positions).await;
            let converted_transactions = native_historical_transactions
                .iter()
                .map({
                    |native_historical_transaction| {
                        HistoricalTransaction {
                            security_name: native_historical_transaction.security_name.to_owned(),
                            security_name_subtext: native_historical_transaction.security_name_subtext.to_owned(),
                            date: native_historical_transaction.date.to_owned(),
                            unit_cost: native_historical_transaction.unit_cost,
                            quantity: native_historical_transaction.quantity,
                            cost: native_historical_transaction.cost,
                            transaction_type: native_historical_transaction.transaction_type.to_owned()
                        }
                    }
                })
            .collect::<Vec<HistoricalTransaction>>();
            converted_transactions
        };
        converted_transactions
    }
}
