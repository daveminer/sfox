use serde_derive::Deserialize;

pub static OPEN_ORDER_FEED: &str = "private.user.open-orders";

#[derive(Debug, Deserialize)]
pub struct OrderPayload {
    pub id: usize,
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

pub fn open_order_feed() -> String {
    OPEN_ORDER_FEED.into()
}
