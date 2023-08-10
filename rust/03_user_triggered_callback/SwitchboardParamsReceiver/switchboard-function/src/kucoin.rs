// Note: Binance API requires a non-US IP address

use crate::*;

use rust_decimal::prelude::*;
use serde::Deserialize;
use serde_json::Value;
pub use switchboard_utils::reqwest;

// https://docs.kucoin.com/#get-all-tickers
// https://api.kucoin.com/api/v1/market/allTickers
#[derive(Debug, Deserialize, Clone)]
pub struct KucoinTicker {
    pub symbol: Pair,
    pub symbolName: String,
    pub buy: Decimal,
    pub sell: Decimal,
    pub changeRate: Decimal,
    pub changePrice: Option<Decimal>,
    pub high: Decimal,
    pub low: Decimal,
    pub vol: Decimal,
    pub volValue: Decimal,
    pub last: Decimal,
    pub averagePrice: Option<Value>,
    pub takerFeeRate: Decimal,
    pub makerFeeRate: Decimal,
    pub takerCoefficient: Decimal,
    pub makerCoefficient: Decimal,
}

impl Into<NormalizedBook> for KucoinTicker {
    fn into(self) -> NormalizedBook {
        let book = self;
        let mut res = NormalizedBook::default();
        if let Some(avg) = book.averagePrice {
            res.price = Decimal::from_str(&avg.to_string()).unwrap_or(book.last);
        }
        res.price = book.last;
        res
    }
}

#[derive(Debug, Deserialize)]
pub struct KucoinTickerResponseInner {
    pub time: i64,
    pub ticker: Vec<KucoinTicker>,
}

#[derive(Debug, Deserialize)]
pub struct KucoinTickerResponse {
    pub code: String,
    pub data: KucoinTickerResponseInner,
}
