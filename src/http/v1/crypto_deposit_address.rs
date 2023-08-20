use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoDepositAddress {
    pub address: String,
    pub currency: String,
}

const CRYPTO_DEPOSIT_ADDRESS_RESOURCE: &str = "user/deposit/address";

impl Client {
    pub fn crypto_deposit_address(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<HashMap<String, Vec<CryptoDepositAddress>>, HttpError>> {
        let url = self.url_for_v1_resource(&currency_path(currency));
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn new_crypto_deposit_address(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<CryptoDepositAddress, HttpError>> {
        let url = self.url_for_v1_resource(&currency_path(currency));
        self.request(HttpVerb::Post, &url, Some(&HashMap::new()))
    }
}

fn currency_path(currency: &str) -> String {
    format!("{}/{}", CRYPTO_DEPOSIT_ADDRESS_RESOURCE, currency)
}
