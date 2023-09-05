use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

static ORDERS_RESOURCE: &str = "orders";
static OPEN_ORDERS_RESOURCE: &str = "orders/open";

#[derive(Clone, Debug, Deserialize)]
pub enum CancellationStatus {
    Canceled,
    #[serde(rename = "Cancel pending")]
    Pending,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelledOrderResponse {
    pub orders: Vec<CancelledOrder>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelledOrder {
    pub id: Option<usize>,
    pub status: CancellationStatus,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoDepositAddress {
    pub address: String,
    pub currency: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewOrder {
    pub id: usize,
    pub quantity: f64,
    pub price: f64,
    pub o_action: String,
    pub pair: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub vwap: f64,
    pub filled: f64,
    pub status: String,
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
    ) -> impl Future<Output = Result<NewOrder, HttpError>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const ORDER_RESPONSE_BODY: &str = r#"
        {"id": 3, "status": "Cancel pending"}
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

    const NEW_ORDER_RESPONSE_BODY: &str = r#"
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

    #[tokio::test]
    async fn test_cancel_all_orders() {
        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: ORDERS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", OPEN_ORDERS_RESOURCE),
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
    async fn test_cancel_order() {
        let id = 2;

        let mock = ApiMock {
            action: HttpVerb::Delete,
            body: ORDER_RESPONSE_BODY.into(),
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
            body: NEW_ORDER_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", ORDERS_RESOURCE, side),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .place_order(side, "ethusd", 0.123, 0.456, "NetPrice", 100, "123A".into())
            .await;

        println!("RESULT: {:?}", result);
        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    // TODO: Implement
    // #[tokio::test]
    // async fn test_order_status() {
    //     let mock = ApiMock {
    //         action: HttpVerb::Post,
    //         body: NEW_ORDER_RESPONSE_BODY.into(),
    //         path: format!("/v1/{}/{}", ORDERS_RESOURCE, side),
    //         response_code: 200,
    //     };

    //     let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

    //     let result = client
    //         .order_status(side, "ethusd", 0.123, 0.456, "NetPrice", 100, "123A".into())
    //         .await;

    //     println!("RESULT: {:?}", result);
    //     assert!(result.is_ok());

    //     for mock in mock_results {
    //         mock.assert_async().await;
    //     }
    // }
}
