use super::Trading212;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentTax {
    fill_id: String,
    name: String,
    quantity: f32,
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
    amount: f32,
    amount_in_euro: f32,
    gross_amount_per_share: f32,
    paid_on: String,
    quantity: f32,
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
    date_executed: String,
    date_modified: String,
    executor: String,
    fill_cost: f32,
    fill_id: i64,
    fill_price: f32,
    fill_result: f32,
    fill_item_type: String,
    filled_quantity: f32,
    filled_value: f32,
    id: i64,
    limit_price: f32,
    ordered_quantity: f32,
    ordered_value: f32,
    parent_order: i64,
    status: String,
    stop_price: f32,
    taxes: Vec<InstrumentTax>,
    ticker: String,
    time_validity: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    amount: f32,
    date_time: String,
    reference: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrderList {
    orders: Vec<HistoricalOrder>,
    next_page_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDividendItemList {
    orders: Vec<HistoricalDividendItem>,
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
