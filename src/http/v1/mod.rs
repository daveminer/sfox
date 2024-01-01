use serde::{
    de::{Deserializer, Unexpected},
    Deserialize,
};

use super::Client;

/// Access user account balance.
pub mod account_balance;
/// Request an ACH transfer.
pub mod ach_bank_transfer;
/// View and create addresses for depositing cryptocurrencies.
pub mod crypto_deposit_address;
/// Currency and currency pair information.
pub mod currency;
/// Custody and approval management.
pub mod custody;
/// Fee information, including withdrawal fees.
pub mod fee;
/// Order management actions and information.
pub mod order;
/// Order book data.
pub mod order_book;
/// If enabled, post-trade settlement features are available here including position, settlement,
/// and risk information.
pub mod post_trade_settlement;
/// Request for a quote to trade with SFox at a predetermined price.
pub mod quote;
/// Reporting on transactions, orders, and monthly summary.
pub mod report;
/// Manage short positions, settlement, and portfolio risk.
pub mod short;
/// Staking information and actions.
pub mod staking;
/// Volume information, total and per-exchange.
pub mod volume;
/// Request withdrawal of funds.
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
