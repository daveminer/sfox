use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoDepositAddress {
    pub address: String,
    pub currency: String,
}

static CRYPTO_DEPOSIT_ADDRESS_RESOURCE: &str = "user/deposit/address";

impl Client {
    pub fn crypto_deposit_address(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<Vec<CryptoDepositAddress>, HttpError>> {
        let url = self.url_for_v1_resource(&currency_path(currency));
        self.request::<Vec<CryptoDepositAddress>>(HttpVerb::Get, &url, None)
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const GET_RESPONSE_BODY: &str = r#"
        [
            {
                "address": "0x123456789",
                "currency": "eth"
            },
            {
                "address": "0x987654321",
                "currency": "btc"
            }
        ]
    "#;

    const POST_RESPONSE_BODY: &str = r#"
        {
            "address": "0x123456789",
            "currency": "eth"
        }
    "#;

    #[tokio::test]
    async fn test_crypto_deposit_address() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: GET_RESPONSE_BODY.into(),
            path: format!("/v1/{}/btc", CRYPTO_DEPOSIT_ADDRESS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.crypto_deposit_address("btc").await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_new_crypto_deposit_address() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: POST_RESPONSE_BODY.into(),
            path: format!("/v1/{}/btc", CRYPTO_DEPOSIT_ADDRESS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.new_crypto_deposit_address("btc").await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
