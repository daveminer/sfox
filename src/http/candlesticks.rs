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
    pub volume: f64,
    pub start_time: usize,
    pub pair: String,
    pub candle_period: usize,
    pub vwap: f64,
    pub trades: usize,
}

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
            "?pair={}&startTime={}&endTime={}&period={}",
            pair, start_time, end_time, period_seconds
        );

        let url = self.url_for_candlestick_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::server::{new_test_server_and_client, ApiMock};

    use super::*;

    const CANDLESTICKS_RESPONSE_BODY: &str = r#"
    [
        {
          "open_price": 9654,
          "high_price": 9662.37,
          "low_price": 9653.66,
          "close_price": 9655.73,
          "volume": 6.31945755,
          "start_time": 1592939280,
          "pair": "btcusd",
          "candle_period": 60,
          "vwap": 9655.70504211,
          "trades": 53
        }
      ]
"#;

    #[tokio::test]
    async fn test_candlesticks() {
        let pair = "btcusd";
        let timestamp = 1000000;
        let period = 3600;
        let day_before = timestamp - 86400;

        let mock = ApiMock {
            action: HttpVerb::Get,
            body: CANDLESTICKS_RESPONSE_BODY.into(),
            // candlestick path resource is missing to match test server
            path: format!(
                "/candlesticks?pair={}&startTime={}&endTime={}&period={}",
                pair, day_before, timestamp, period
            ),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let response = client
            .candlesticks("btcusd", day_before, timestamp, period)
            .await;

        assert!(response.is_ok());

        let candles = response.unwrap();

        assert_eq!(candles.len(), 1);

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
