use serde::Deserialize;
use serde_derive::Serialize;

pub mod account;
pub mod market;

#[derive(Debug, Deserialize)]
pub struct WsResponse<T> {
    pub recipient: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
pub struct WsSystemResponse<T> {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeMsg {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub feeds: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct WsSubscriptionResponsePayload {
    pub action: String,
    pub feeds: Vec<String>,
}

pub enum SubscribeAction {
    Subscribe,
    Unsubscribe,
}

impl From<SubscribeAction> for String {
    fn from(val: SubscribeAction) -> Self {
        match val {
            SubscribeAction::Subscribe => "subscribe".to_string(),
            SubscribeAction::Unsubscribe => "unsubscribe".to_string(),
        }
    }
}
