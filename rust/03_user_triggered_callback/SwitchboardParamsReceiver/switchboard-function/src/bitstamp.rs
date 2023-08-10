// Note: Binance API requires a non-US IP address

use crate::*;

use serde::Deserialize;
pub use switchboard_utils::reqwest;

// https://www.bitstamp.net/api/v2/ticker/
// https://www.bitstamp.net/api/#tag/Tickers/operation/GetCurrencies
#[derive(Debug, Deserialize, Clone)]
pub struct BitstampTicker {
    pub timestamp: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub last: Decimal,
    pub volume: Decimal,
    pub vwap: Decimal,
    pub bid: Decimal,
    pub ask: Decimal,
    pub side: Decimal,
    pub open_24: Decimal,
    pub percent_change_24: Option<String>,
    pub pair: Pair,
}

impl Into<NormalizedBook> for BitstampTicker {
    fn into(self) -> NormalizedBook {
        let book = self;
        let mut res = NormalizedBook::default();
        res.price = book.last;
        res
    }
}
