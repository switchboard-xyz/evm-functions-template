// Note: Binance API requires a non-US IP address

use crate::*;

pub use switchboard_utils::reqwest;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct CoinbaseBook {
    pub bids: Vec<(String, String, i64)>,
    pub asks: Vec<(String, String, i64)>,
}
impl Into<NormalizedBook> for CoinbaseBook {
    fn into(self) -> NormalizedBook {
        let book = self;
        let mut res = NormalizedBook::default();
        for bid in book.bids.iter() {
            res.bids.push(NormalizedOrdersRow {
                price: Decimal::try_from(bid.0.as_str()).unwrap(),
                amount: Decimal::try_from(bid.1.as_str()).unwrap(),
            });
        }
        for ask in book.asks.iter() {
            res.asks.push(NormalizedOrdersRow {
                price: Decimal::try_from(ask.0.as_str()).unwrap(),
                amount: Decimal::try_from(ask.1.as_str()).unwrap(),
            });
        }
        res.price = res.bids[0]
            .price
            .checked_add(res.asks[0].price)
            .unwrap()
            .checked_div(2.into())
            .unwrap();
        res
    }
}

// https://api.coinbase.com/v2/exchange-rates?currency=USD
#[allow(non_snake_case)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct CoinbaseSpontResponseInternal {
    pub currency: String,
    pub rates: HashMap<String, Decimal>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct CoinbaseSpotResponse {
    pub data: CoinbaseSpontResponseInternal,
}
