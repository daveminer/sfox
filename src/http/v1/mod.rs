use serde::{
    de::{Deserializer, Unexpected},
    Deserialize,
};

use super::Client;

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
pub mod report;
pub mod short;
pub mod staking;
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_bool_from_int() {
        // Test 0 -> false
        let result = bool_from_int(json!(0)).unwrap();
        assert_eq!(result, false);

        // Test 1 -> true
        let result = bool_from_int(json!(1)).unwrap();
        assert_eq!(result, true);

        // Test invalid value errors
        let result = bool_from_int(json!(2));
        assert!(result.is_err());
    }
}
