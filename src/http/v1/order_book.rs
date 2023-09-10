use futures_util::Future;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::Client;
use crate::http::{HttpError, HttpVerb};

static ORDER_BOOK_RESOURCE: &str = "markets/orderbook";

#[derive(Clone, Debug, Deserialize)]
pub struct OrderBook {
    pub pair: String,
    pub currency: Option<String>,
    pub asks: Vec<OpenOrder>,
    pub bids: Vec<OpenOrder>,
    pub market_making: MarketMaking,
    //pub timestamps: HashMap<String, Vec<usize>>,
    pub lastupdated: usize,
    pub lastpublished: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarketMaking {
    pub asks: Vec<OpenOrder>,
    pub bids: Vec<OpenOrder>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenOrder {
    pub price: f64,
    #[serde(deserialize_with = "maybe_sci_notation_to_f64")]
    pub volume: f64,
    pub exchange: String,
}

fn maybe_sci_notation_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => {
            let reformatted = s.replace('e', "E");
            reformatted.parse::<f64>().map_err(serde::de::Error::custom)
        }
        Value::Number(n) => Ok(n.as_f64().unwrap()),
        _ => Err(serde::de::Error::custom("Expected string or number")),
    }
}

impl Client {
    pub fn order_book(self, pair: &str) -> impl Future<Output = Result<OrderBook, HttpError>> {
        let query_str = format!("{}/{}", ORDER_BOOK_RESOURCE, pair);
        let url = self.url_for_v1_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const ORDER_BOOK_RESPONSE_BODY: &str = r#"
        {
            "bids": [
                [
                    9458.12,
                    "1e-08",
                    "gemini"
                ],
                [
                    9456,
                    1,
                    "itbit"
                ],
                [
                    9453,
                    0.73553115,
                    "itbit"
                ]
            ],
            "asks": [
                [
                    9455.55,
                    2.03782954,
                    "market1"
                ],
                [
                    9455.56,
                    0.9908,
                    "market1"
                ],
                [
                    9455.59,
                    0.60321264,
                    "market1"
                ]
            ],
            "market_making": {
                "bids": [
                    [
                        9447.34,
                        2,
                        "bitstamp"
                    ],
                    [
                        9452.01,
                        0.60321264,
                        "market1"
                    ],
                    [
                        9452.31,
                        0.47488688,
                        "bittrex"
                    ],
                    [
                        9456,
                        1,
                        "itbit"
                    ],
                    [
                        9458.12,
                        "1e-08",
                        "gemini"
                    ]
                ],
                "asks": [
                    [
                        9458.13,
                        2.07196048,
                        "gemini"
                    ],
                    [
                        9457.75,
                        0.14748797,
                        "itbit"
                    ],
                    [
                        9456,
                        0.1686167,
                        "bittrex"
                    ],
                    [
                        9455.68,
                        0.742406,
                        "bitstamp"
                    ],
                    [
                        9455.55,
                        2.03782954,
                        "market1"
                    ]
                ]
            },
            "timestamps": {
                "gemini": [
                    1572903458537,
                    1572903458538
                ],
                "bitstamp": [
                    1572903458199,
                    1572903458199
                ],
                "itbit": [
                    1572903458414,
                    1572903458416
                ],
                "bittrex": [
                    1572903458517,
                    1572903458517
                ],
                "market1": [
                    1572903458071,
                    1572903458071
                ]
            },
            "lastupdated": 1572903458756,
            "pair": "btcusd",
            "currency": "usd",
            "lastpublished": 1572903458798
        }
    "#;

    #[tokio::test]
    async fn test_order_book() {
        let pair = "ethusd";

        let mock = ApiMock {
            action: HttpVerb::Get,
            body: ORDER_BOOK_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", ORDER_BOOK_RESOURCE, pair),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.order_book(pair).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
