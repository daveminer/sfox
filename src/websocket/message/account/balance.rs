use serde_derive::Deserialize;

use super::super::WsResponse;

pub type WsBalanceResponse = WsResponse<WsBalanceResponsePayload>;

pub static BALANCE_FEED: &str = "private.user.balances";

#[derive(Debug, Deserialize)]
pub struct WsBalanceResponsePayload {
    pub enabled: String,
    pub equity: String,
    pub equity_for_withdrawals: String,
    pub available_exposure: String,
    pub exposure: String,
    pub exposure_limit: String,
}

pub fn balance_feed() -> String {
    BALANCE_FEED.into()
}
