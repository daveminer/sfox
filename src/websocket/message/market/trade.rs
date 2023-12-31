use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq)]
pub struct Trade {
    #[serde(rename = "buyOrderId")]
    pub buy_order_id: String,
    #[serde(rename = "sellOrderId")]
    pub sell_order_id: String,
    pub pair: String,
    pub pair_id: usize,
    pub price: String,
    pub quantity: String,
    pub side: String,
    pub exchange: String,
    pub exchange_id: usize,
    pub timestamp: String,
    pub is_decimal: bool,
}

pub fn trades_feed(basequote: &str) -> String {
    format!("trades.sfox.{}", basequote)
}
