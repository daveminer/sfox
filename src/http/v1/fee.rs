use futures_util::Future;
use serde::Deserialize;

use super::super::{HttpError, HttpVerb, SFox};

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

impl SFox {
    pub fn fees(self) -> impl Future<Output = Result<Fees, HttpError>> {
        let url = self.url_for_v1_resource("account/fee-rates");
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn withdraw_fee(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<WithdrawFee, HttpError>> {
        let url = self.url_for_v1_resource(&format!("withdraw-fee/{}", currency));
        self.request(HttpVerb::Get, &url, None)
    }
}
