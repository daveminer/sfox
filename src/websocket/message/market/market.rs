use serde_derive::Deserialize;

use super::WsResponse;

pub type WsBalanceResponse = WsResponse<WsBalanceResponsePayload>;
pub type WsOrdersResponse = WsResponse<WsOrdersResponsePayload>;

#[derive(Debug, Deserialize)]
pub struct WsBalanceResponsePayload {
    pub enabled: String,
    pub equity: String,
    pub equity_for_withdrawals: String,
    pub available_exposure: String,
    pub exposure: String,
    pub exposure_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct WsOrdersResponsePayload {
    pub id: String,
    pub client_order_id: String,
    pub status: String,
    pub filled: String,
    pub filled_amount: String,
    pub vwap: String,
    pub price: String,
    pub quantity: String,
    pub pair: String,
    pub action: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub algorithm_id: usize,
    pub fees: String,
}
