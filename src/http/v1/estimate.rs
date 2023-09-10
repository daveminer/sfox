use futures_util::Future;
use serde::Deserialize;

use super::Client;
use crate::http::{HttpError, HttpVerb};

static ORDER_ESTIMATE_RESOURCE: &str = "offer";

#[derive(Clone, Debug, Deserialize)]
pub enum RoutingType {
    NetPrice,
    Smart,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Estimate {
    pub price: f64,
    pub subtotal: f64,
    pub fees: f64,
    pub total: f64,
    pub quantity: f64,
    pub vwap: f64,
    pub currency_pair: String,
    pub routing_type: RoutingType,
}

impl Client {
    // TODO: quantity OR maxspend required; not necessarily both
    pub fn order_estimate(
        self,
        side: &str,
        pair: &str,
        quantity: f64,
        maxspend: f64,
        routing_type: &str,
    ) -> impl Future<Output = Result<Estimate, HttpError>> {
        let query_str = format!(
            "{}/{}?pair={}&quantity={}&maxspend={}&routing_type={}",
            ORDER_ESTIMATE_RESOURCE, side, pair, quantity, maxspend, routing_type
        );
        let url = self.url_for_v1_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const RESPONSE_BODY: &str = r#"
        {
            "price": 19203.23787426,
            "subtotal": 19202.05116807,
            "fees": 6.72071791,
            "total": 19208.77188598,
            "quantity": 1,
            "vwap": 19202.05116807,
            "currency_pair": "btcusd",
            "routing_type": "NetPrice"
        }
    "#;

    #[tokio::test]
    async fn test_order_estimate() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: RESPONSE_BODY.into(),
            path: format!(
                "/v1/{}/{}?pair={}&quantity={}&maxspend={}&routing_type={}",
                ORDER_ESTIMATE_RESOURCE, "buy", "ethusd", 0.5, 1.0, "Smart"
            ),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .order_estimate("buy", "ethusd", 0.5, 1.0, "Smart")
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
