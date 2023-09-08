use futures_util::Future;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;

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

        let url = self.url_for_v1_resource("quote");

        Box::pin(self.request(HttpVerb::Post, &url, Some(&params)))
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

        let url = self.url_for_v1_resource("orders/buy");

        self.request(HttpVerb::Post, &url, Some(&params))
    }
}
