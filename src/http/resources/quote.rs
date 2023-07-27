use futures_util::Future;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;

use super::bool_from_int;

use crate::http::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct Quote {
    pub quote_id: String,
    pub quantity: f64,
    pub amount: f64,
    pub pair: String,
    pub side: String,
    pub date_expiry: String,
    pub date_quote: String,
    pub buy_price: Option<f64>,
    pub sell_price: Option<f64>,
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
    #[serde(deserialize_with = "bool_from_int")]
    pub filled: bool,
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
    pub quote_id: String,
}

impl Client {
    pub fn request_for_quote(
        self,
        pair: &str,
        side: &str,
        quantity: Option<f64>,
        amount: Option<f64>,
        client_quote_id: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Quote, HttpError>>>> {
        let mut params = HashMap::new();
        params.insert("pair".into(), pair.to_string());
        params.insert("side".into(), side.to_string());

        if quantity.is_none() && amount.is_none() {
            return Box::pin(async {
                Err(HttpError::InvalidRequest(
                    "Either quantity or amount must be provided".into(),
                ))
            });
        }

        if let Some(quantity) = quantity {
            params.insert("quantity".into(), quantity.to_string());
        }

        if let Some(amount) = amount {
            params.insert("amount".into(), amount.to_string());
        }

        if let Some(client_quote_id) = client_quote_id {
            params.insert("client_quote_id".into(), client_quote_id.to_string());
        }

        Box::pin(self.request(HttpVerb::Post, "quote", Some(&params)))
    }

    pub fn execute_quote(
        self,
        currency_pair: &str,
        quantity: f64,
        quote_id: &str,
    ) -> impl Future<Output = Result<Quote, HttpError>> {
        let mut params = HashMap::new();
        params.insert("currency_pair".into(), currency_pair.to_string());
        params.insert("quantity".into(), quantity.to_string());
        params.insert("quote_id".into(), quote_id.to_string());

        self.request(HttpVerb::Post, "orders/buy", Some(&params))
    }
}
