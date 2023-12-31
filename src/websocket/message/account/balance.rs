use serde::{Deserialize, Deserializer};

pub static BALANCE_FEED: &str = "private.user.balances";

#[derive(Debug, Deserialize)]
pub struct BalancePayload {
    pub currency: String,
    #[serde(deserialize_with = "str_to_f64")]
    pub balance: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub available: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub held: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub trading_wallet: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub collateral_wallet: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub borrow_wallet: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub lending_wallet: f64,
}

pub fn balance_feed() -> String {
    BALANCE_FEED.into()
}

// Custom deserialization function to convert string to f64
fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}
