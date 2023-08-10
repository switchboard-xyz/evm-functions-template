// Note: Binance API requires a non-US IP address

use crate::*;

pub use switchboard_utils::reqwest;

use serde::Deserialize;
use serde::Serialize;

// https://api.huobi.pro/market/tickers
#[derive(Deserialize, Debug, Clone)]
pub struct HuobiTicker {
    pub symbol: Pair,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub amount: f64,
    pub vol: f64,
    pub count: u32,
    pub bid: f64,
    pub bidSize: f64,
    pub ask: f64,
    pub askSize: f64,
}

// receive the following data
// { data: [{"symbol":"sylousdt","open":0.001357,"high":0.001366,"low":0.001355,"close":0.001364,"amount":4.40982312269891E8,"vol":597364.4924611878,"count":13300,"bid":0.00136,"bidSize":37348.8287,"ask":0.001378,"askSize":38438.1536}]}

// write a type to receive the following data

#[derive(Deserialize, Debug)]
pub struct HuobiTickerResponse {
    pub data: Vec<HuobiTicker>,
}

impl Into<NormalizedTicker> for HuobiTicker {
    fn into(self) -> NormalizedTicker {
        let book = self;
        let mut res = NormalizedTicker::default();
        res
    }
}
