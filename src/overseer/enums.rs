use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Asset {
    Stock(Stock),
    Crypto(Crypto),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock {
    pub ticker: String,
    pub isin: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crypto {
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TransactionType {
    Buy,
    Sell,
    Dividend,
    Deposit,
    Withdrawal,
    Fee,
    Unknown,
}