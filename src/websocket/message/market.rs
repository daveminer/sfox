use serde_derive::Deserialize;

use crate::websocket::SFoxWs;

#[derive(Deserialize)]
pub struct WsResponse<T> {
    pub recipient: String,
    pub sequence: usize,
    pub timestamp: usize,
    pub payload: T,
}

#[derive(Deserialize)]
pub struct WsSubscriptionResponse {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: WsSubscriptionResponsePayload,
    pub recipient: String,
    pub sequence: usize,
    pub timestamp: usize,
}

#[derive(Deserialize)]
pub struct WsSubscriptionResponsePayload {
    pub available: String,
    pub balance: String,
    pub borrow_wallet: String,
    pub currency: String,
    pub collateral_wallet: String,
    pub held: String,
    pub trading_wallet: String,
}

impl SFoxWs {
    // pub async fn subscribe_to_order_book(
    //     &mut self,
    //     pair: &str,
    //     fee_adjusted: bool,
    // ) -> Result<(), HttpError> {
    //     let feed = if fee_adjusted { "net" } else { "sfox" };

    //     let msg = format!("orderbook.{}.{}", feed, pair);

    //     self.subscribe([msg]).await
    // }
}
