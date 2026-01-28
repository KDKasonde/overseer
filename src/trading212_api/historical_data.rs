use super::Trading212;
use rust_decimal::Decimal;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentTax {
    fill_id: String,
    name: String,
    quantity: Decimal,
    time_charged: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataIncluded {
    include_dividends: bool,
    include_interest: bool,
    include_orders: bool,
    include_transactions: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDividendItem {
    amount: Decimal,
    amount_in_euro: Decimal,
    gross_amount_per_share: Decimal,
    paid_on: String,
    quantity: Decimal,
    reference: String,
    ticker: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportItem {
    data_included: ReportDataIncluded,
    download_link: String,
    report_id: i64,
    status: String,
    time_from: String,
    time_to: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrder {
    date_created: String,
    pub date_executed: String,
    date_modified: String,
    executor: String,
    fill_cost: Decimal,
    fill_id: i64,
    fill_price: Decimal,
    fill_result: Decimal,
    fill_item_type: String,
    pub filled_quantity: Decimal,
    pub filled_value: Decimal,
    id: i64,
    pub limit_price: Decimal,
    ordered_quantity: Decimal,
    ordered_value: Decimal,
    parent_order: i64,
    status: String,
    stop_price: Decimal,
    taxes: Vec<InstrumentTax>,
    pub ticker: String,
    time_validity: String,
    pub item_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    amount: Decimal,
    date_time: String,
    reference: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrderList {
    pub orders: Vec<HistoricalOrder>,
    next_page_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDividendItemList {
    pub orders: Vec<HistoricalDividendItem>,
    next_page_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportItemList {
    items: Vec<ExportItem>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionList {
    items: Vec<Transaction>,
    next_page_path: String,
}

impl Trading212 {
    pub async fn fetch_historical_orders(&self, cursor: Option<i64>, ticker: &str, limit: Option<i64>) -> HistoricalOrderList {
        let client = &self.client;
        let target_url = format!("{}equity/history/orders", self.base_url );
       
        let cursor = match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    ..= 0 => 1,
                    50.. => 50,
                    _ => value
                }
            },
            None => 20
        };

        let res = client
            .get(target_url)
            .query(&[("cursor", cursor), ("limit", limit)])
            .query(&[("ticker", ticker)])
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<HistoricalOrderList>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        
        return output
    }

    pub async fn fetch_paid_dividends(&self, cursor: Option<i64>, ticker: &str, limit: Option<i64>) -> HistoricalDividendItemList {
        let client = &self.client;
        let target_url = format!("{}history/dividends", self.base_url );

        let cursor = match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    ..= 0 => 1,
                    50.. => 50,
                    _ => value
                }
            },
            None => 20
        };

        let res = client
            .get(target_url)
            .query(&[("cursor", cursor), ("limit", limit)])
            .query(&[("ticker", ticker)])
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<HistoricalDividendItemList>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        return output
    }


    pub async fn fetch_transaction_list(&self, cursor: Option<i64>, limit: Option<i64>) -> TransactionList {
        let client = &self.client;
        let target_url = format!("{}history/transactions", self.base_url );

        let cursor = match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    ..= 0 => 1,
                    50.. => 50,
                    _ => value
                }
            },
            None => 20
        };

        let res = client
            .get(target_url)
            .query(&[("cursor", cursor), ("limit", limit)])
            .send()
            .await;
        
        let res = match res {
            Ok(response) => { 
                response
                    .json::<TransactionList>()
                    .await
            },
            Err(error)  => {
                // This should not panic unless there is something wrong with auth, the url or the
                // headers.
                panic!("Response was not okay! Received the following error: \n\t{}", error);
            }
        }; 
        
        let output = match res {
            Ok(response) => { 
                response
            },
            Err(error)  => {
                panic!("Derserialization failed, error: \n\t{}", error);
            }
        }; 
        
        return output
    }

}
