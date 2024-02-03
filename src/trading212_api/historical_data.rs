use super::Trading212;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstrumentTax {
    fillId: String,
    name: String,
    quantity: f32,
    timeCharged: String,
}

#[derive(Debug, Deserialize)]
pub struct ReportDataIncluded {
    includeDividends: bool,
    includeInterest: bool,
    includeOrders: bool,
    includeTransactions: bool,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalDividendItem {
    amount: f32,
    amountInEuro: f32,
    grossAmountPerShare: f32,
    paidOn: String,
    quantity: f32,
    reference: String,
    ticker: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportItem {
    dataIncluded: ReportDataIncluded,
    downloadLink: String,
    reportId: i64,
    status: String,
    timeFrom: String,
    timeTo: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalOrder {
    dateCreated: String,
    dateExecuted: String,
    dateModified: String,
    executor: String,
    fillCost: f32,
    fillId: i64,
    fillPrice: f32,
    fillResult: f32,
    fillitem_type: String,
    filledQuantity: f32,
    filledValue: f32,
    id: i64,
    limitPrice: f32,
    orderedQuantity: f32,
    orderedValue: f32,
    parentOrder: i64,
    status: String,
    stopPrice: f32,
    taxes: Vec<InstrumentTax>,
    ticker: String,
    timeValidity: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    amount: f32,
    dateTime: String,
    reference: String,
    item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalOrderList {
    orders: Vec<HistoricalOrder>,
    next_page_path: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalDividendItemList {
    orders: Vec<HistoricalDividendItem>,
    next_page_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportItemList {
    items: Vec<ExportItem>
}

#[derive(Debug, Deserialize)]
pub struct TransactionList {
    items: Vec<Transaction>,
    next_page_path: String,
}

impl Trading212 {
    pub async fn fetch_historical_orders(&self, cursor: Option<i64>, ticker: &str, limit: Option<i64>) -> Result<HistoricalOrderList, reqwest::Error>{
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
        
        let output = match res {
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
        return output
    }

    pub async fn fetch_paid_dividends(&self, cursor: Option<i64>, ticker: &str, limit: Option<i64>) -> Result<HistoricalDividendItemList, reqwest::Error> {
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
        
        let output = match res {
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
        return output
    }


    pub async fn fetch_transaction_list(&self, cursor: Option<i64>, limit: Option<i64>) -> Result<TransactionList, reqwest::Error> {
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
        
        let output = match res {
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
        return output
    }

}
