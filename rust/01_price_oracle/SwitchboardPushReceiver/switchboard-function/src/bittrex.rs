// Note: Binance API requires a non-US IP address

use crate::*;

use serde::Deserialize;
pub use switchboard_utils::reqwest;

//https://api.bittrex.com/v3/markets/tickers
#[derive(Debug, Deserialize, Clone)]
pub struct BittrexPair {
    pub symbol: Pair,
    pub lastTradeRate: Decimal,
    pub bidRate: Decimal,
    pub askRate: Decimal,
    pub updatedAt: String,
}

impl Into<NormalizedTicker> for BittrexPair {
    fn into(self) -> NormalizedTicker {
        let book = self;
        let mut res = NormalizedTicker::default();
        res.price = (book.bidRate + book.askRate) / Decimal::from(2);
        res
    }
}
