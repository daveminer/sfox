use serde_derive::Deserialize;

use crate::websocket::message::WsResponse;

pub type WsTradesResponse = WsResponse<WsTrade>;

#[derive(Debug, Deserialize, PartialEq)]
pub enum TransactionSide {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WsTrade {
    #[serde(rename = "buyOrderId")]
    pub buy_order_id: String,
    #[serde(rename = "sellOrderId")]
    pub sell_order_id: String,
    pub pair: String,
    pub pair_id: usize,
    pub price: f64,
    pub quantity: f64,
    pub side: TransactionSide,
    pub exchange: String,
    pub exchange_id: usize,
    pub timestamp: String,
    pub is_decimal: bool,
}

pub fn trades_feed(basequote: &str) -> String {
    format!("trades.sfox.{}", basequote)
}
