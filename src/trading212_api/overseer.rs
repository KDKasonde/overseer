use crate::{
    overseer::{
        enums::{Asset, Stock, TransactionType},
        errors::OverseerError,
        structs::{Account, HistoricalTransaction, Position},
        traits::{OverseenAccount, ReadableSecurity},
    },
    trading212_api::portfolio_data::OpenPosition,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::Trading212;

impl ReadableSecurity for OpenPosition {
    fn get_asset(&self) -> Asset {
        Asset::Stock(Stock {
            ticker: self.ticker.clone(),
            isin: "".to_string(),
            name: "".to_string(),
        })
    }

    fn get_source(&self) -> String {
        "Trading 212".to_string()
    }
}

#[async_trait(?Send)]
impl OverseenAccount for Trading212 {
    async fn get_cash(&self) -> Result<Account, OverseerError> {
        let native_account = self.fetch_account_cash().await?;
        let blocked = native_account.blocked.unwrap_or_default();
        Ok(Account {
            source: "Trading 212".to_string(),
            blocked,
            free: native_account.free,
            total_funds: native_account.free + native_account.pie_cash + blocked,
            invested: native_account.invested,
            ppl: native_account.ppl,
            total: native_account.total,
        })
    }

    async fn get_asset_summary(&self) -> Vec<Position> {
        let native_summary = self.fetch_portfolio_positions().await;

        native_summary
            .iter()
            .map(|native_position| {
                let total_value = native_position.quantity * native_position.current_price;
                let total_cost = native_position.quantity * native_position.average_price;
                let ppl = total_value - total_cost;
                let ppl_as_perc = if total_cost != Decimal::ZERO {
                    (ppl / total_cost) * Decimal::new(100, 2)
                } else {
                    Decimal::ZERO
                };

                Position {
                    source: "Trading 212".to_string(),
                    asset: Asset::Stock(Stock {
                        ticker: native_position.ticker.clone(),
                        isin: "".to_string(),
                        name: "".to_string(),
                    }),
                    total_value,
                    total_cost,
                    current_price: native_position.current_price,
                    ppl,
                    ppl_as_perc,
                    quantity: native_position.quantity,
                }
            })
            .collect::<Vec<Position>>()
    }

    async fn get_historical_transactions(
        &self,
        position: Box<dyn ReadableSecurity>,
    ) -> Vec<HistoricalTransaction> {
        let asset = position.get_asset();
        let ticker = match &asset {
            Asset::Stock(stock) => &stock.ticker,
            Asset::Crypto(crypto) => &crypto.symbol,
        };

        let native_historical_transactions =
            self.fetch_historical_orders(None, ticker, None).await;

        native_historical_transactions
            .orders
            .iter()
            .map(|native_historical_transaction| {
                let transaction_type = match native_historical_transaction.item_type.as_str() {
                    "MARKET_BUY" => TransactionType::Buy,
                    "MARKET_SELL" => TransactionType::Sell,
                    _ => TransactionType::Fee, // Default or handle other types
                };

                let date_executed: DateTime<Utc> =
                    DateTime::parse_from_rfc3339(&native_historical_transaction.date_executed)
                        .unwrap()
                        .into();

                HistoricalTransaction {
                    asset: asset.clone(),
                    date: date_executed,
                    unit_price: native_historical_transaction.limit_price,
                    quantity: native_historical_transaction.filled_quantity,
                    total_value: native_historical_transaction.filled_value,
                    transaction_type,
                }
            })
            .collect::<Vec<HistoricalTransaction>>()
    }
}