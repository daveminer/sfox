use futures_util::Future;
use serde::Deserialize;

use super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct Candle {
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub volume: usize,
    pub start_time: String,
    pub pair: String,
    pub candle_period: String,
    pub vwap: f64,
    pub trades: usize,
}

static SERVER: &str = "https://chartdata.sfox.com";

impl Client {
    // Responses are limited to 500 candles.
    pub fn candlesticks(
        self,
        pair: &str,
        start_time: usize,
        end_time: usize,
        period_seconds: usize,
    ) -> impl Future<Output = Result<Vec<Candle>, HttpError>> {
        let query_str = format!(
            "candlesticks?pair={}&startTime={}&endTime={}&period={}",
            pair, start_time, end_time, period_seconds
        );
        let url = format!("{}/{}", SERVER, query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}
