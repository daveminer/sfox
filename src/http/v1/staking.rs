use std::collections::HashMap;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{HttpError, HttpVerb, SFox};

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
    pub data: Vec<StakingCurrency>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StakingTransaction {
    pub amount: f64,
    pub atx_id: usize,
    pub stake_start: Option<String>,
    pub stake_end: Option<String>,
    pub staked_reward_amount: f64,
    pub staked_auto_restake: f64,
    pub date_added: String,
    pub date_updated: String,
    pub currency_symbol: String,
    pub status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub stake_bonding_period_minutes: usize,
    pub stake_unbonding_period_minutes: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnstakeResponse {
    pub data: Vec<Unstake>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Unstake {
    pub id: usize,
}

impl SFox {
    pub fn staking_currencies(
        self,
    ) -> impl Future<Output = Result<StakingCurrenciesResponse, HttpError>> {
        let query_str = self.url_for_v1_resource("staking/currencies");
        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn staking_transactions(
        self,
    ) -> impl Future<Output = Result<StakingTransactionsResponse, HttpError>> {
        let query_str = self.url_for_v1_resource("staking/transactions");
        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn stake(
        self,
        currency: String,
        quantity: f64,
    ) -> impl Future<Output = Result<(), HttpError>> {
        let query_str = self.url_for_v1_resource("staking/stake");

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        params.insert("quantity".to_string(), quantity.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }

    pub fn unstake(
        self,
        currency: String,
        quantity: f64,
    ) -> impl Future<Output = Result<(), HttpError>> {
        let query_str = self.url_for_v1_resource("staking/unstake");

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        params.insert("quantity".to_string(), quantity.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }
}
