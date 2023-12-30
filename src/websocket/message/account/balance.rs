use serde_derive::Deserialize;

pub static BALANCE_FEED: &str = "private.user.balances";

#[derive(Debug, Deserialize)]
pub struct BalancePayload {
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
