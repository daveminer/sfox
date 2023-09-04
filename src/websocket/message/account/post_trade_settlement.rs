use serde_derive::Deserialize;

use super::super::WsResponse;

pub type WsPostTradeSettlementResponse = WsResponse<WsPostTradeSettlementResponsePayload>;

static POST_TRADE_SETTLEMENT_FEED: &str = "private.user.post-trade-settlement";

#[derive(Debug, Deserialize)]
pub struct WsPostTradeSettlementResponsePayload {
    pub enabled: String,
    pub equity: String,
    pub equity_for_withdrawals: String,
    pub available_exposure: String,
    pub exposure: String,
    pub exposure_limit: String,
}

pub fn post_trade_settlement_feed() -> String {
    POST_TRADE_SETTLEMENT_FEED.into()
}
