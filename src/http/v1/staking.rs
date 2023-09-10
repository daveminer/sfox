use std::collections::HashMap;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{Client, HttpError, HttpVerb};

static STAKING_CURRENCIES_RESOURCE: &str = "staking/currencies";
static STAKING_TRANSACTIONS_RESOURCE: &str = "staking/transactions";
static STAKE_RESOURCE: &str = "staking/stake";
static UNSTAKE_RESOURCE: &str = "staking/unstake";

#[derive(Clone, Debug, Deserialize)]
pub struct StakingCurrenciesResponse {
    pub data: Vec<StakingCurrency>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StakingCurrency {
    pub currency: String,
    pub min_stake_amount: String,
    pub min_stake_period_minutes: Option<usize>,
    pub stake_bonding_period_minutes: usize,
    pub stake_unbonding_period_minutes: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StakingTransactionsResponse {
    pub data: Vec<StakingTransaction>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StakingTransaction {
    pub amount: f64,
    pub atx_id: usize,
    pub stake_start: Option<String>,
    pub stake_end: Option<String>,
    pub staked_reward_amount: Option<f64>,
    pub staked_auto_restake: Option<f64>,
    pub date_added: String,
    pub date_updated: String,
    pub currency_symbol: String,
    pub status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnstakeResponse {
    pub data: Unstake,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Unstake {
    pub id: usize,
}

impl Client {
    pub fn staking_currencies(
        self,
    ) -> impl Future<Output = Result<StakingCurrenciesResponse, HttpError>> {
        let query_str = self.url_for_v1_resource(STAKING_CURRENCIES_RESOURCE);
        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn staking_transactions(
        self,
    ) -> impl Future<Output = Result<StakingTransactionsResponse, HttpError>> {
        let query_str = self.url_for_v1_resource(STAKING_TRANSACTIONS_RESOURCE);
        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn stake(
        self,
        currency: String,
        quantity: f64,
    ) -> impl Future<Output = Result<(), HttpError>> {
        let query_str = self.url_for_v1_resource(STAKE_RESOURCE);

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        params.insert("quantity".to_string(), quantity.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }

    pub fn unstake(
        self,
        currency: String,
        quantity: f64,
    ) -> impl Future<Output = Result<UnstakeResponse, HttpError>> {
        let query_str = self.url_for_v1_resource(UNSTAKE_RESOURCE);

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        params.insert("quantity".to_string(), quantity.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const STAKING_CURRENCIES_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "currency": "avax",
                    "min_stake_amount": "25",
                    "min_stake_period_minutes": 20160,
                    "stake_bonding_period_minutes": 0,
                    "stake_unbonding_period_minutes": 0
                }
            ]
        }
    "#;

    const STAKING_TRANSACTIONS_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "amount": 0.0670929,
                    "atx_id": 1452808,
                    "stake_start": null,
                    "stake_end": null,
                    "staked_reward_amount": null,
                    "staked_auto_restake": null,
                    "date_added": "2022-09-29T23:05:34.000Z",
                    "date_updated": "2022-09-29T23:05:34.000Z",
                    "currency_symbol": "avax",
                    "status": "done",
                    "type": "reward"
                },
                {
                    "amount": 25,
                    "atx_id": 1315723,
                    "stake_start": "2022-09-15T23:04:27.000Z",
                    "stake_end": "2022-09-29T23:04:27.000Z",
                    "staked_reward_amount": 0.0670929,
                    "staked_auto_restake": 0,
                    "date_added": "2022-09-15T23:04:11.000Z",
                    "date_updated": "2022-09-29T23:05:34.000Z",
                    "currency_symbol": "avax",
                    "status": "Unstaked",
                    "type": "stake"
                }
            ]
        }
    "#;

    const UNSTAKE_RESPONSE_BODY: &str = r#"
        {
            "data": {
                "id": 807278
            }
        }
    "#;

    #[tokio::test]
    async fn test_staking_currencies() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: STAKING_CURRENCIES_RESPONSE_BODY.into(),
            path: format!("/v1/{}", STAKING_CURRENCIES_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.staking_currencies().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_staking_transactions() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: STAKING_TRANSACTIONS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", STAKING_TRANSACTIONS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.staking_transactions().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_stake() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: "null".into(),
            path: format!("/v1/{}", STAKE_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.stake("avax".into(), 12.1).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_unstake() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: UNSTAKE_RESPONSE_BODY.into(),
            path: format!("/v1/{}", UNSTAKE_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.unstake("avax".into(), 12.1).await;
        println!("RESULT: {:?}", result);
        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
