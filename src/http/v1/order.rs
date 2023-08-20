use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub enum CancellationStatus {
    Canceled,
    Pending,
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

impl Client {
    pub fn cancel_all_orders(self) -> impl Future<Output = Result<Vec<CancelledOrder>, HttpError>> {
        let url = self.url_for_v1_resource("orders/open");
        self.request(HttpVerb::Delete, &url, None)
    }

    pub fn cancel_order(
        self,
        order_id: usize,
    ) -> impl Future<Output = Result<CancelledOrder, HttpError>> {
        let url = self.url_for_v1_resource(&format!("orders/{}", order_id));
        self.request(HttpVerb::Delete, &url, None)
    }

    pub fn cancel_orders(
        self,
        order_ids: Vec<usize>,
    ) -> impl Future<Output = Result<Vec<CancelledOrder>, HttpError>> {
        // Create a comma separated list of order ids from the vector
        let order_ids_query_param = order_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = self.url_for_v1_resource(&format!("orders?ids={}", order_ids_query_param));

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
    ) -> impl Future<Output = Result<HashMap<String, Vec<CryptoDepositAddress>>, HttpError>> {
        let mut params = HashMap::new();
        params.insert("currency_pair".to_string(), currency_pair.to_string());
        params.insert("price".to_string(), price.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("routing_type".to_string(), routing_type.to_string());
        params.insert("algorithm_id".to_string(), algorithm_id.to_string());
        if let Some(client_order_id) = client_order_id {
            params.insert("client_order_id".to_string(), client_order_id.to_string());
        }

        let url = self.url_for_v1_resource(&format!("orders/{}", side));

        self.request(HttpVerb::Post, &url, Some(&params))
    }
}
