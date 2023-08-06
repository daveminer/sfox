use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::http::HttpError;

pub mod market;
pub mod orders;

static DEFAULT_WS_SERVER_URL: &str = "wss://ws.sfox.com/ws";

#[derive(Debug)]
pub struct SFoxWs {
    // #[serde(skip)]
    // pub auth_token: String,
    pub server_url: String,
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeMsg {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub feeds: Vec<String>,
}

impl SFoxWs {
    pub async fn new(server_url: Option<&str>) -> Result<SFoxWs, HttpError> {
        let server_url = match server_url {
            Some(url) => url.into(),
            None => {
                env::var("SFOX_WS_SERVER_URL").unwrap_or_else(|_| DEFAULT_WS_SERVER_URL.to_string())
            }
        };

        let (stream, response) = match connect_async(server_url.clone()).await {
            Ok((stream, response)) => (stream, response),
            Err(e) => {
                return Err(HttpError::InitializationError(format!(
                    "Could not connect to websocke server: {}",
                    e.to_string()
                )))
            }
        };

        if !response.status().is_informational() {
            return Err(HttpError::InitializationError(format!(
                "Could not connect to websocket: {:?}",
                response
            )));
        }

        // let auth_token = env::var("SFOX_AUTH_TOKEN").map_err(|_| {
        //     HttpError::InitializationError("SFOX_AUTH_TOKEN env variable not set.".to_string())
        // })?;

        let (write, read) = stream.split();

        Ok(SFoxWs {
            // auth_token,
            server_url,
            write,
            read,
        })
    }
}
