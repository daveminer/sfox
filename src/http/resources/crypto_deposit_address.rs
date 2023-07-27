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
        self.request(
            HttpVerb::Get,
            &format!("{}/{}", CRYPTO_DEPOSIT_ADDRESS_RESOURCE, currency),
            None,
        )
    }

    pub fn new_crypto_deposit_address(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<CryptoDepositAddress, HttpError>> {
        self.request(
            HttpVerb::Post,
            &format!("{}/{}", CRYPTO_DEPOSIT_ADDRESS_RESOURCE, currency),
            Some(&HashMap::new()),
        )
    }
}
