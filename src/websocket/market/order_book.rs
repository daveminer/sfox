use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeMsg {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub feeds: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderBookType {
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "sfox")]
    SFox,
}
