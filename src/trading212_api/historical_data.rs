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
    type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportItem {
    dataIncluded: ReportDataIncluded,
    downloadLink: String,
    reportId: int64,
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
    fillId: int64,
    fillPrice: f32,
    fillResult: f32,
    fillType: String,
    filledQuantity: f32,
    filledValue: f32,
    id: int64,
    limitPrice: f32,
    orderedQuantity: f32,
    orderedValue: f32,
    parentOrder: int64,
    status: String,
    stopPrice: f32,
    taxes: Vec<InstrumentTax>,
    ticker: String,
    timeValidity: String,
    type: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    amount: f32,
    dateTime: String,
    reference: String,
    type: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalOrderList {
    orders: Vec<HistoricalOrder>
    nextPagePath: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalDividendItemList {
    orders: Vec<HistoricalDividendItem>
    nextPagePath: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportItemList {
    items: Vec<ExportItem>
}

#[derive(Debug, Derserialize)]
pub struct TransactionList {
    items: Vec<Transaction>,
    nextPagePath: String,
}

impl Trading212 {
    pub async fn fetch_historical_orders(&self, cursor: Option<int64>, ticker: &str, limit: Option<int64>) -> Result<HistoricalOrderList, reqwest::Error>{
        let client = &self.client;
        let target_url = format!("{}equity/history/orders", self.base_url );
       
        let cursor - match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    value < 0 => 1,
                    value > 50 => 50,
                    _ => value
                }
            },
            None => 20
        };

        let res = client
            .get(target_url)
            .query(&[("cursor", cursor), ("ticker", ticker), ("limit", limit)])
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

    pub async fn fetch_paid_dividends(&self, cursor: Option<int64>, ticker: &str, limit: Option<int64>) -> Result<HistoricalDividendItemList, reqwest::Error> {
        let client = &self.client;
        let target_url = format!("{}equity/account/portfolio/{ticker}", self.base_url );

        let cursor - match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    value < 0 => 1,
                    value > 50 => 50,
                    _ => value
                }
            },
            None => 20
        };

        let res = client
            .get(target_url)
            .query(&[("cursor", cursor), ("ticker", ticker), ("limit", limit)])
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


    pub async fn fetch_transaction_list(&self, cursor: Option<int64>, limit: Option<int64>) -> Result<TransactionList, reqwest::Error> {
        let client = &self.client;
        let target_url = format!("{}equity/account/portfolio/{ticker}", self.base_url );

        let cursor - match cursor {
            Some(value) => value,
            None => 1
        };

        let limit = match limit {
            Some(value) => {
                match value {
                    value < 0 => 1,
                    value > 50 => 50,
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
