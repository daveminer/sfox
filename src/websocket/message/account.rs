use serde_derive::Deserialize;

pub static OPEN_ORDER_FEED: &str = "private.user.open-orders";
pub static BALANCE_FEED: &str = "private.user.balances";
pub static POST_TRADE_SETTLEMENT_FEED: &str = "private.user.post-trade-settlement";

#[derive(Deserialize)]
pub struct WsBalanceResponsePayload {
    pub enabled: String,
    pub equity: String,
    pub equity_for_withdrawals: String,
    pub available_exposure: String,
    pub exposure: String,
    pub exposure_limit: String,
}

#[derive(Deserialize)]
pub struct WsPostTradeSettlementResponsePayload {
    pub enabled: String,
    pub equity: String,
    pub equity_for_withdrawals: String,
    pub available_exposure: String,
    pub exposure: String,
    pub exposure_limit: String,
}

#[derive(Deserialize)]
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

// enum WsAccountMsg {
//     Auth,
//     Order,
// }

// impl WsAccountMsg {
//     fn value(&self) -> &'static str {
//         match self {
//             WsAccountMsg::Auth => "authenticate",
//             WsAccountMsg::Order => "order",
//         }
//     }

//     fn feed_name(&self) -> &'static str {
//         match self {
//             WsAccountMsg::Auth => "auth",
//         }
//     }
// }

// #[derive(Deserialize)]
// pub struct WsPostTradeSettlementResponse {
//     pub recipient: String,
//     pub sequence: usize,
//     pub timestamp: usize,
//     pub payload: WsPostTradeSettlementResponsePayload,
// }
