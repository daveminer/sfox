use futures_util::Future;
use serde::Deserialize;

use super::SFox;
use crate::http::{HttpError, HttpVerb};

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

impl SFox {
    pub fn order_estimate(
        self,
        side: &str,
        pair: &str,
        quantity: f64,
        maxspend: f64,
        routing_type: &str,
    ) -> impl Future<Output = Result<Estimate, HttpError>> {
        let query_str = format!(
            "offer/{}?pair={}&quantity={}&maxspend={}&routing_type={}",
            side, pair, quantity, maxspend, routing_type
        );
        let url = self.url_for_v1_resource(&query_str);
        println!("URL: {}", url);

        self.request(HttpVerb::Get, &url, None)
    }
}
