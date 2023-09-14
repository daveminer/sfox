use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

static DONE_ORDERS_RESOURCE: &str = "orders/done";
static LIST_ASSET_PAIRS_RESOURCE: &str = "markets/currency_pairs";
static OPEN_ORDERS_RESOURCE: &str = "orders/open";
static ORDERS_RESOURCE: &str = "orders";

#[derive(Clone, Debug, Deserialize)]
pub enum OrderStatus {
    Started,
    #[serde(rename = "Cancel pending")]
    Pending,
    Canceled,
    Filled,
    Done,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetPair {
    pub formatted_symbol: String,
    pub symbol: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelledOrderResponse {
    pub orders: Vec<CancelledOrder>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelledOrder {
    pub id: Option<usize>,
    pub status: OrderStatus,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoDepositAddress {
    pub address: String,
    pub currency: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExecutedQuote {
    pub id: usize,
    pub side_id: usize,
    pub action: String,
    pub algorithm_id: usize,
    pub algorithm: String,
    #[serde(rename = "type")]
    pub execution_type: String,
    pub pair: String,
    pub quantity: f64,
    pub price: f64,
    pub amount: f64,
    pub net_market_amount: f64,
    pub filled: f64,
    pub vwap: f64,
    pub filled_amount: f64,
    pub fees: f64,
    pub net_proceeds: f64,
    pub status: String,
    pub status_code: usize,
    pub routing_option: String,
    pub routing_type: String,
    pub time_in_force: String,
    pub expires: Option<String>,
    pub dateupdated: String,
    pub client_order_id: Option<String>,
    pub user_tx_id: Option<String>,
    pub o_action: String,
    pub algo_id: usize,
    pub algorithm_options: Option<String>,
    pub destination: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Order {
    pub id: usize,
    pub quantity: f64,
    pub price: f64,
    pub o_action: String,
    pub pair: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub vwap: f64,
    pub filled: f64,
    pub status: OrderStatus,
}

impl Client {
    pub fn cancel_all_orders(
        self,
    ) -> impl Future<Output = Result<CancelledOrderResponse, HttpError>> {
        let url = self.url_for_v1_resource(&format!("{}", OPEN_ORDERS_RESOURCE));
        self.request(HttpVerb::Delete, &url, None)
    }

    pub fn cancel_order(
        self,
        order_id: usize,
    ) -> impl Future<Output = Result<CancelledOrder, HttpError>> {
        let url = self.url_for_v1_resource(&format!("{}/{}", ORDERS_RESOURCE, order_id));
        self.request(HttpVerb::Delete, &url, None)
    }

    pub fn cancel_orders(
        self,
        order_ids: Vec<usize>,
    ) -> impl Future<Output = Result<CancelledOrderResponse, HttpError>> {
        // Create a comma separated list of order ids from the vector
        let order_ids_query_param = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = self.url_for_v1_resource(&format!(
            "{}?ids={}",
            ORDERS_RESOURCE, order_ids_query_param
        ));

        self.request(HttpVerb::Delete, &url, None)
    }

    pub fn open_orders(self) -> impl Future<Output = Result<Vec<Order>, HttpError>> {
        let url = self.url_for_v1_resource(&format!("{}", ORDERS_RESOURCE));
        self.request(HttpVerb::Get, &url, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn place_order(
        self,
        side: &str,
        currency_pair: &str,
        price: f64,
        quantity: f64,
        routing_type: &str,
        algorithm_id: usize,
        client_order_id: Option<&str>,
    ) -> impl Future<Output = Result<Order, HttpError>> {
        let mut params = HashMap::new();
        params.insert("currency_pair".to_string(), currency_pair.to_string());
        params.insert("price".to_string(), price.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("routing_type".to_string(), routing_type.to_string());
        params.insert("algorithm_id".to_string(), algorithm_id.to_string());
        if let Some(client_order_id) = client_order_id {
            params.insert("client_order_id".to_string(), client_order_id.to_string());
        }

        let url = self.url_for_v1_resource(&format!("{}/{}", ORDERS_RESOURCE, side));

        self.request(HttpVerb::Post, &url, Some(&params))
    }

    pub fn order_status(self, order_id: &str) -> impl Future<Output = Result<Order, HttpError>> {
        let url = self.url_for_v1_resource(&format!("{}/{}", ORDERS_RESOURCE, order_id));

        self.request(HttpVerb::Get, &url, None)
    }

    pub fn done_orders(self) -> impl Future<Output = Result<Vec<ExecutedQuote>, HttpError>> {
        let url = self.url_for_v1_resource(DONE_ORDERS_RESOURCE);

        self.request(HttpVerb::Get, &url, None)
    }

    pub fn list_asset_pairs(
        self,
    ) -> impl Future<Output = Result<HashMap<String, AssetPair>, HttpError>> {
        let url = self.url_for_v1_resource(LIST_ASSET_PAIRS_RESOURCE);

        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const CANCEL_PENDING_ORDER_RESPONSE_BODY: &str = r#"
        {"id": 3, "status": "Cancel pending"}
    "#;

    const CANCEL_MULTIPLE_ORDERS_FAILED_RESPONSE_BODY: &str = r#"
        { "error": "the order ids provided were invalid or the orders were already done/canceled" }
    "#;

    const OPEN_ORDERS_RESPONSE_BODY: &str = r#"
        [
            {
                "id": 123,
                "quantity": 1,
                "price": 10,
                "o_action": "Buy",
                "pair": "btcusd",
                "type": "Limit",
                "vwap": 0,
                "filled": 0,
                "status": "Started"
            },
            {
                "id": 456,
                "quantity": 1,
                "price": 10,
                "o_action": "Sell",
                "pair": "btcusd",
                "type": "Limit",
                "vwap": 0,
                "filled": 0,
                "status": "Started"
            }
        ]
    "#;

    const ORDERS_RESPONSE_BODY: &str = r#"
        {
            "orders": [
                {"id": 2, "status": "Cancel pending"},
                {"id": 3, "status": "Canceled"},
                {"id": 4, "status": "Canceled"}
            ]
        }
    "#;

    const ORDER_RESPONSE_BODY: &str = r#"
        {
            "id": 123,
            "quantity": 1,
            "price": 10,
            "o_action": "Buy",
            "pair": "btcusd",
            "type": "Limit",
            "vwap": 0,
            "filled": 0,
            "status": "Started"
        }
    "#;

    const DONE_ORDERS_RESPONSE_BODY: &str = r#"
        [
            {
                "id": 701968334,
                "side_id": 500,
                "action": "Buy",
                "algorithm_id": 200,
                "algorithm": "Smart",
                "type": "Smart",
                "pair": "btcusd",
                "quantity": 0.001,
                "price": 16900.6,
                "amount": 0,
                "net_market_amount": 0,
                "filled": 0.001,
                "vwap": 16900.6,
                "filled_amount": 16.9006,
                "fees": 0.0338012,
                "net_proceeds": -16.8667988,
                "status": "Done",
                "status_code": 300,
                "routing_option": "BestPrice",
                "routing_type": "NetPrice",
                "time_in_force": "GTC",
                "expires": null,
                "dateupdated": "2022-11-18T01:26:40.000Z",
                "client_order_id": "94b0e7c4-0fa7-403d-a0d0-6c4ccec76630",
                "user_tx_id": "94b0e7c4-0fa7-403d-a0d0-6c4ccec76630",
                "o_action": "Buy",
                "algo_id": 200,
                "algorithm_options": null,
                "destination": ""
            },
            {
                "id": 701945645,
                "side_id": 500,
                "action": "Buy",
                "algorithm_id": 201,
                "algorithm": "Limit",
                "type": "Limit",
                "pair": "btcusd",
                "quantity": 0.01,
                "price": 16905,
                "amount": 0,
                "net_market_amount": 0,
                "filled": 0.01,
                "vwap": 16643,
                "filled_amount": 166.43,
                "fees": 0.16643,
                "net_proceeds": -166.26357,
                "status": "Done",
                "status_code": 300,
                "routing_option": "BestPrice",
                "routing_type": "NetPrice",
                "time_in_force": "GTC",
                "expires": null,
                "dateupdated": "2022-11-17T19:39:18.000Z",
                "client_order_id": "",
                "user_tx_id": "",
                "o_action": "Buy",
                "algo_id": 201,
                "algorithm_options": null,
                "destination": ""
            }
        ]
    "#;

    const LIST_ASSET_PAIRS_RESPONSE_BODY: &str = r#"
        {
            "bchbtc": {
                "formatted_symbol": "BCH/BTC",
                "symbol": "bchbtc"
            },
            "bchusd": {
                "formatted_symbol": "BCH/USD",
                "symbol": "bchusd"
            },
            "btcusd": {
                "formatted_symbol": "BTC/USD",
                "symbol": "btcusd"
            }
        }
    "#;

    #[tokio::test]
    async fn test_cancel_all_orders() {
        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: ORDERS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", DONE_ORDERS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.cancel_all_orders().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_cancel_multiple_orders() {
        let ids = vec![2, 3];
        let ids_param = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: ORDERS_RESPONSE_BODY.into(),
            path: format!("/v1/{}?ids={}", ORDERS_RESOURCE, ids_param),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.cancel_orders(vec![2, 3]).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_cancel_multiple_orders_failed() {
        let ids = vec![2, 3];
        let ids_param = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: CANCEL_MULTIPLE_ORDERS_FAILED_RESPONSE_BODY.into(),
            path: format!("/v1/{}?ids={}", ORDERS_RESOURCE, ids_param),
            response_code: 400,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.cancel_orders(vec![2, 3]).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                == "Invalid request: `\"the order ids provided were invalid or the orders were already done/canceled\"`"
        );

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let id = 2;

        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: CANCEL_PENDING_ORDER_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", ORDERS_RESOURCE, id),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.cancel_order(id).await;
        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_place_order() {
        let side = "sell";

        let mock = ApiMock {
            action: HttpVerb::Post,
            body: ORDER_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", ORDERS_RESOURCE, side),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .place_order(side, "ethusd", 0.123, 0.456, "NetPrice", 100, "123A".into())
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_order_status() {
        let order_id = "abc";
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: ORDER_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", ORDERS_RESOURCE, order_id),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.order_status(order_id).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_open_orders() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: OPEN_ORDERS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", ORDERS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.open_orders().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_done_orders() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: DONE_ORDERS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", DONE_ORDERS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.done_orders().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_list_asset_pairs() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: LIST_ASSET_PAIRS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", LIST_ASSET_PAIRS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.list_asset_pairs().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
