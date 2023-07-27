use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct Fees {
    pub volume: f64,
    #[serde(rename = "makerRate")]
    pub maker_rate: f64,
    #[serde(rename = "nprRate")]
    pub npr_rate: f64,
    #[serde(rename = "nprOffRate")]
    pub npr_off_rate: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WithdrawFee {
    pub fee: f64,
}

impl Client {
    pub fn fees(self) -> impl Future<Output = Result<Fees, HttpError>> {
        self.request(HttpVerb::Get, "account/fee-rates", None)
    }

    pub fn withdraw_fee(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<WithdrawFee, HttpError>> {
        self.request(HttpVerb::Get, &format!("withdraw-fee/{}", currency), None)
    }
}
