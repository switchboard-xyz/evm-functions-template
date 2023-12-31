// Note: Binance API requires a non-US IP address

use crate::*;

use serde::Deserialize;
pub use switchboard_utils::reqwest;

// https://poloniex.com/public?command=returnTicker
#[derive(Debug, Deserialize, Clone)]
pub struct PoloniexTicker {
    pub id: i64,
    pub last: Decimal,
    pub lowestAsk: Decimal,
    pub highestBid: Decimal,
    pub percentChange: Decimal,
    pub baseVolume: Decimal,
    pub quoteVolume: Decimal,
    pub isFrozen: Decimal,
    pub postOnly: Decimal,
    pub high24hr: Decimal,
    pub low24hr: Decimal,
}

impl Into<NormalizedTicker> for PoloniexTicker {
    fn into(self) -> NormalizedTicker {
        let book = self;
        let mut res = NormalizedTicker::default();
        res.price = book.last;
        res
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PoloniexResponse(HashMap<Pair, PoloniexTicker>);

impl PoloniexResponse {
    pub fn into_inner(self) -> HashMap<Pair, PoloniexTicker> {
        self.0
    }
}
