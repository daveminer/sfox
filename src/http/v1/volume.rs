use futures_util::Future;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

use super::Client;
use crate::http::{HttpError, HttpVerb};

static VOLUME_RESOURCE: &str = "analytics/volume";

#[derive(Clone, Debug, Deserialize)]
pub enum Interval {
    Minute,
    Hour,
    Day,
}

#[derive(Clone, Debug)]

pub enum VolumeRecord {
    ExchangeVolumeResponse(ExchangeVolumeResponse),
    VolumeResponse(VolumeResponse),
}

#[derive(Clone, Debug, Deserialize)]
pub struct VolumeResponse {
    pub data: Vec<TotalVolumeRecord>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeVolumeResponse {
    pub data: Vec<ExchangeVolumeList>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeVolumeList {
    pub timestamp: usize,
    pub volumes: Vec<ExchangeVolumeRecord>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeVolumeRecord {
    pub exchange: String,
    pub volume: f64,
    pub usd_notional: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TotalVolumeRecord {
    pub timestamp: usize,
    pub volume: f64,
    pub usd_notional: f64,
}

impl<'de> Deserialize<'de> for VolumeRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the JSON as a generic value.
        let value: Value = Deserialize::deserialize(deserializer)?;
        let data: &Value = match value.get("data") {
            Some(data) => data,
            None => {
                return Err(serde::de::Error::custom(
                    "Missing 'data' field in VolumeRecord",
                ))
            }
        };

        let first_data_elem = match data.as_array() {
            Some(data) => {
                if data.len() == 0 {
                    return Err(serde::de::Error::custom(
                        "Empty 'data' field in VolumeRecord",
                    ));
                }
                data[0].clone()
            }
            None => {
                return Err(serde::de::Error::custom(
                    "Invalid 'data' field in VolumeRecord",
                ))
            }
        };

        match first_data_elem.get("volumes") {
            Some(_volumes) => {
                // Deserialize it as ExchangeVolumeResponse
                let exchange_volume_response: ExchangeVolumeResponse =
                    serde_json::from_value(value.clone()).map_err(serde::de::Error::custom)?;
                Ok(VolumeRecord::ExchangeVolumeResponse(
                    exchange_volume_response,
                ))
            }
            None => {
                // Deserialize it as VolumeResponse
                let volume_response: VolumeResponse =
                    serde_json::from_value(value.clone()).map_err(serde::de::Error::custom)?;
                Ok(VolumeRecord::VolumeResponse(volume_response))
            }
        }
    }
}
impl Client {
    pub fn volume(
        self,
        start_time: usize,
        end_time: usize,
        interval: Interval,
        currency: &str,
        net: bool,
        by_exchange: bool,
    ) -> impl Future<Output = Result<VolumeRecord, HttpError>> {
        let query_str = format!(
            "{}?start_time={}&end_time={}&interval={}&currency={}&net={}&by_exchange={}",
            VOLUME_RESOURCE,
            start_time,
            end_time,
            convert_interval(interval),
            currency,
            net,
            by_exchange
        );
        let url = self.url_for_v1_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}

fn convert_interval<'a>(interval: Interval) -> &'a str {
    match interval {
        Interval::Minute => "60",
        Interval::Hour => "3600",
        Interval::Day => "86400",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_test_server_and_client, ApiMock};

    const GROSS_VOLUME_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "timestamp": 1689206400000,
                    "volume": 324557.08579094,
                    "usd_notional": 633266040.1379151
                },
                {
                    "timestamp": 1689292800000,
                    "volume": 244525.03601467,
                    "usd_notional": 482590637.1359154
                },
                {
                    "timestamp": 1689379200000,
                    "volume": 54552.86708131,
                    "usd_notional": 105565737.85291174
                },
                {
                    "timestamp": 1689465600000,
                    "volume": 59350.88216805,
                    "usd_notional": 114595675.74899396
                },
                {
                    "timestamp": 1689552000000,
                    "volume": 174930.26792864,
                    "usd_notional": 333229267.7529286
                }
            ]
        }
    "#;

    const NET_VOLUME_BY_EXCHANGE_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "timestamp": 1689206400000,
                    "volumes": [
                        {
                            "exchange": "bitfinex",
                            "volume": -6825.69751025,
                            "usd_notional": -13318096.02295827
                        },
                        {
                            "exchange": "bitflyer",
                            "volume": 405.5878231,
                            "usd_notional": 791370.7816200266
                        },
                        {
                            "exchange": "bitstamp",
                            "volume": 2621.19358698,
                            "usd_notional": 5114394.218867672
                        },
                        {
                            "exchange": "coinbase",
                            "volume": 25602.95423207,
                            "usd_notional": 49955715.503370605
                        },
                        {
                            "exchange": "kraken",
                            "volume": 1303.36668958,
                            "usd_notional": 2543089.9477870227
                        }
                    ]
                },
                {
                    "timestamp": 1689292800000,
                    "volumes": [
                        {
                            "exchange": "bitfinex",
                            "volume": -3128.46017994,
                            "usd_notional": -6174278.168395847
                        },
                        {
                            "exchange": "bitflyer",
                            "volume": 81.1377265,
                            "usd_notional": 160132.098396033
                        },
                        {
                            "exchange": "bitstamp",
                            "volume": 211.54172883,
                            "usd_notional": 417495.3181104044
                        },
                        {
                            "exchange": "coinbase",
                            "volume": 4753.17507186,
                            "usd_notional": 9380789.074742645
                        },
                        {
                            "exchange": "gemini",
                            "volume": -341.430339,
                            "usd_notional": -673841.3682337883
                        },
                        {
                            "exchange": "kraken",
                            "volume": -1088.1927376,
                            "usd_notional": -2147639.5019672103
                        }
                    ]
                }
            ]
        }
    "#;

    const UNAUTHORIZED_RESPONSE_BODY: &str = r#"
        {
            "error": "invalid token. check authorization header."
        }
    "#;

    const UNPROCESSABLE_ENTITY_RESPONSE_BODY: &str = r#"
        {
            "error": "currency must be a non-blank string"
        }
    "#;

    #[tokio::test]
    async fn test_total_volume() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: GROSS_VOLUME_RESPONSE_BODY.into(),
            path: format!(
                "/v1/{}?start_time=1694374019&end_time=1694384019&interval=3600&currency=btc&net=false&by_exchange=false",
                VOLUME_RESOURCE
            ),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .volume(1694374019, 1694384019, Interval::Hour, "btc", false, false)
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_volume_by_exchange() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: NET_VOLUME_BY_EXCHANGE_RESPONSE_BODY.into(),
            path: format!("/v1/{}?start_time=1694374019&end_time=1694384019&interval=3600&currency=btc&net=true&by_exchange=true", VOLUME_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .volume(1694374019, 1694384019, Interval::Hour, "btc", true, true)
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_volume_unauthorized() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: UNAUTHORIZED_RESPONSE_BODY.into(),
            path: format!("/v1/{}?start_time=1694374019&end_time=1694384019&interval=3600&currency=btc&net=true&by_exchange=true", VOLUME_RESOURCE),
            response_code: 401,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .volume(1694374019, 1694384019, Interval::Hour, "btc", true, true)
            .await;

        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string()
                == "Invalid request: `\"invalid token. check authorization header.\"`"
        );

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_volume_unprocessable_entity() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: UNPROCESSABLE_ENTITY_RESPONSE_BODY.into(),
            path: format!("/v1/{}?start_time=1694374019&end_time=1694384019&interval=3600&currency=&net=true&by_exchange=true", VOLUME_RESOURCE),
            response_code: 422,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .volume(1694374019, 1694384019, Interval::Hour, "", true, true)
            .await;

        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string()
                == "Invalid request: `\"currency must be a non-blank string\"`"
        );

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
