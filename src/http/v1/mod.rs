use serde::{
    de::{Deserializer, Unexpected},
    Deserialize,
};

use super::SFox;

pub mod account_balance;
pub mod ach_bank_transfer;
pub mod crypto_deposit_address;
pub mod currency;
pub mod custody;
pub mod estimate;
pub mod fee;
pub mod order;
pub mod order_book;
pub mod post_trade_settlement;
pub mod quote;
pub mod short;
pub mod staking;
pub mod transaction_history;
pub mod volume;
pub mod withdraw;

pub(crate) fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

impl SFox {
    pub(crate) fn url_for_v1_resource(&self, resource: &str) -> String {
        format!("{}/v1/{}", self.server_url, resource)
    }
}
