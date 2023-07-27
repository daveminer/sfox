use futures_util::Future;
use serde::Deserialize;
use std::collections::HashMap;

use super::super::{Client, HttpError, HttpVerb};
use super::bool_from_int;

#[derive(Clone, Debug, Deserialize)]
pub struct Currency {
    pub id: usize,
    pub symbol: String,
    pub name: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_fiat: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_lending_enabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub can_deposit: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub can_withdraw: bool,
    pub min_withdrawal: f64,
    pub confirmations_needed: Option<usize>,
    pub precision: usize,
    pub ascii_sign: String,
    pub contract_address: Option<String>,
    #[serde(deserialize_with = "bool_from_int")]
    pub custody_enabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub trading_enabled: bool,
    pub primary_network: Option<String>,
    pub code: String,
    pub currency: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CurrencyPair {
    pub formatted_symbol: String,
    pub symbol: String,
    pub base: String,
    pub quote: String,
}

impl Client {
    pub fn currencies(self) -> impl Future<Output = Result<Vec<Currency>, HttpError>> {
        self.request(HttpVerb::Get, "currency", None)
    }

    pub fn currency_pairs(
        self,
    ) -> impl Future<Output = Result<HashMap<String, CurrencyPair>, HttpError>> {
        self.request(HttpVerb::Get, "markets/currency-pairs", None)
    }
}
