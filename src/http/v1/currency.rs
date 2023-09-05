use futures_util::Future;
use serde::Deserialize;
use std::collections::HashMap;

use super::super::{Client, HttpError, HttpVerb};
use super::bool_from_int;

static CURRENCIES_RESOURCE: &str = "currency";
static CURRENCY_PAIRS_RESOURCE: &str = "markets/currency-pairs";

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
        let url = self.url_for_v1_resource(CURRENCIES_RESOURCE);
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn currency_pairs(
        self,
    ) -> impl Future<Output = Result<HashMap<String, CurrencyPair>, HttpError>> {
        let url = self.url_for_v1_resource(CURRENCY_PAIRS_RESOURCE);
        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const CURRENCIES_RESPONSE_BODY: &str = r#"
        [
            {
                "id": 1,
                "symbol": "usd",
                "name": "US Dollar",
                "is_fiat": 1,
                "is_lending_enabled": 0,
                "can_deposit": 1,
                "can_withdraw": 1,
                "min_withdrawal": 10,
                "confirmations_needed": null,
                "precision": 8,
                "ascii_sign": "$",
                "contract_address": null,
                "custody_enabled": 1,
                "trading_enabled": 1,
                "primary_network": null,
                "code": "usd",
                "currency": "usd"
            },
            {
                "id": 2,
                "symbol": "btc",
                "name": "Bitcoin",
                "is_fiat": 0,
                "is_lending_enabled": 0,
                "can_deposit": 1,
                "can_withdraw": 1,
                "min_withdrawal": 0.001,
                "confirmations_needed": 3,
                "precision": 8,
                "ascii_sign": "B",
                "contract_address": null,
                "custody_enabled": 1,
                "trading_enabled": 1,
                "primary_network": "Bitcoin",
                "code": "btc",
                "currency": "btc"
            }
        ]
    "#;

    const CURRENCY_PAIRS_RESPONSE_BODY: &str = r#"
        {
            "btcusd": {
                "formatted_symbol": "BTC/USD",
                "symbol": "btcusd",
                "base": "btc",
                "quote": "usd"
            },
            "ethbtc": {
                "formatted_symbol": "ETH/BTC",
                "symbol": "ethbtc",
                "base": "eth",
                "quote": "btc"
            },
            "ltcbtc": {
                "formatted_symbol": "LTC/BTC",
                "symbol": "ltcbtc",
                "base": "ltc",
                "quote": "btc"
            }
        }
    "#;

    #[tokio::test]
    async fn test_currencies() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: CURRENCIES_RESPONSE_BODY.into(),
            path: format!("/v1/{}", CURRENCIES_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.currencies().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_currency_pairs() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: CURRENCY_PAIRS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", CURRENCY_PAIRS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.currency_pairs().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
