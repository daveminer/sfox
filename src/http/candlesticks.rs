use futures_util::Future;
use serde::Deserialize;

use super::{Client, HttpError, HttpVerb};

/// A single element of candlestick chart data returned from the API.
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
    /// Candlestick chart data from the SFox markets.
    /// Responses are limited to 500 candles from the server.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_candlesticks() {
        let _ = env::set_var("SFOX_AUTH_TOKEN", "abc123");
        let client = Client::new().unwrap();

        let timestamp = 1000000; //Utc::now().timestamp_millis() as usize;
        let day_before = timestamp - 86400;

        let response = client
            .candlesticks("btcusd", day_before, timestamp, 3600)
            .await;

        assert!(response.is_ok());

        let candles = response.unwrap();
        println!("{:?}", candles);

        // Add assertions on candles, e.g.:
        assert_eq!(candles.len(), 24);
        assert!(candles[0].start_time < candles[23].start_time);
    }
}
