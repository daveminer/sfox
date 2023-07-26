use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct Withdrawal {
    pub address: String,
    pub amount: f64,
    pub currency: String,
    #[serde(rename = "isWire")]
    pub is_wire: bool,
}

impl Client {
    pub fn withdraw(
        self,
        address: &str,
        amount: f64,
        currency: &str,
        is_wire: bool,
    ) -> impl Future<Output = Result<Withdrawal, HttpError>> {
        let mut params = HashMap::new();
        params.insert("address".to_string(), address.to_string());
        params.insert("amount".to_string(), amount.to_string());
        params.insert("currency".to_string(), currency.to_string());
        params.insert("isWire".to_string(), is_wire.to_string());

        self.request(HttpVerb::Post, "/user/withdraw", Some(&params))
    }
}
