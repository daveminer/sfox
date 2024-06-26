use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ticker {
    pub amount: f64,
    pub exchange: String,
    pub last: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub pair: String,
    pub route: String,
    pub source: String,
    pub timestamp: String,
    pub volume: f64,
    pub vwap: f64,
}

pub fn ticker_feed(basequote: &str) -> String {
    format!("ticker.sfox.{}", basequote)
}
