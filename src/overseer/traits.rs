use super::structs::{Account, HistoricalTransaction, Position};
use std::future::Future;

pub trait OverseenAccount {
    fn get_cash(&self) -> impl Future<Output = Account>;
    fn get_asset_summary (&self ) -> impl Future<Output = Vec<Position>>;
    fn get_historical_transactions (&self, position: impl ReadableSecurity) -> impl Future<Output = Vec<HistoricalTransaction>>;    
    fn get_all_historical_transactions(&self) -> impl Future<Output = Vec<HistoricalTransaction>>; 
}

pub trait LoginRequired {
    fn login(&self) -> impl Future<Output = ()>;
    fn logout(&self) -> impl Future<Output = ()>;
}

pub trait ReadableSecurity {
    fn get_security_id(&self) -> String;
    fn get_security_name(&self) -> String;
    fn get_security_name_subtext(&self) -> Option<String> {
        None
    }
}
