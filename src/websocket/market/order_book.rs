use serde_derive::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::http::HttpError;

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

// impl SFoxWs {
//     pub async fn subscribe(&mut self, feeds: Vec<String>) -> Result<(), HttpError> {
//         let msg = SubscribeMsg {
//             msg_type: "subscribe".to_string(),
//             feeds,
//         };

//         let msg = serde_json::to_string(&msg).map_err(|e| {
//             HttpError::InitializationError(format!("Could not serialize subscribe msg: {}", e))
//         })?;

//         self.write.send(Message::Text(msg)).await.map_err(|e| {
//             HttpError::InitializationError(format!("Could not send subscribe msg: {}", e))
//         })?;

//         Ok(())
//     }
// }
