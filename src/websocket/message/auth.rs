use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct WsAuthResponse {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: WsAuthResponsePayload,
    pub sequence: usize,
    pub timestamp: usize,
}

#[derive(Deserialize)]
pub struct WsAuthResponsePayload {
    pub action: String,
}
