use std::env::{self, VarError};

use serde_derive::Deserialize;
use serde_json::{json, Error};
use tokio_tungstenite::tungstenite::Message;

use super::{message::WsSystemResponse, Client, WebsocketClientError};

#[derive(Debug, Deserialize)]
pub struct WsAuthResponsePayload {
    pub action: String,
}

impl Client {
    /// Authenticate a connected socket
    pub async fn authenticate(&self) -> Result<(), WebsocketClientError> {
        let msg = match auth_message() {
            Ok(msg) => msg,
            Err(e) => return Err(WebsocketClientError::AuthenticationError(e.to_string())),
        };

        match self.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => Err(WebsocketClientError::AuthenticationError(e.to_string())),
        }
    }

    // Validates a message as a successful response to the message sent by authenticate()
    pub fn auth_message_check_success(msg: &str) -> Result<bool, Error> {
        let auth_response: WsSystemResponse<WsAuthResponsePayload> = serde_json::from_str(msg)?;

        Ok(auth_response.msg_type == "success" && auth_response.payload.action == "authenticate")
    }
}

fn auth_message() -> Result<Message, VarError> {
    let auth_token = env::var("SFOX_AUTH_TOKEN")?;

    let msg = json!({
      "type": "authenticate",
      "apiKey": auth_token
    })
    .to_string();

    Ok(Message::Text(msg))
}
