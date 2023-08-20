use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use std::env;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::http::HttpError;

use self::message::{auth::auth_message, SubscribeAction, SubscribeMsg};

pub mod message;

static DEFAULT_WS_SERVER_URL: &str = "wss://ws.sfox.com/ws";

#[derive(Debug)]
pub struct ClientWs {
    pub server_url: String,
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl ClientWs {
    pub async fn new(server_url: Option<&str>) -> Result<ClientWs, HttpError> {
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
                    "Could not connect to websocket server: {}",
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

        let (write, read) = stream.split();

        Ok(ClientWs {
            server_url,
            write,
            read,
        })
    }

    pub async fn authenticate(&mut self) -> Result<(), HttpError> {
        let msg = match auth_message() {
            Ok(msg) => msg,
            Err(e) => return Err(HttpError::InitializationError(e.to_string())),
        };

        match self.write.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::TransportError(e.to_string())),
        }
    }

    pub async fn subscribe(&mut self, feeds: Vec<String>) -> Result<(), HttpError> {
        match self
            .write
            .send(ws_feed_msg(feeds, SubscribeAction::Subscribe))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::TransportError(e.to_string())),
        }
    }

    pub async fn unsubscribe(&mut self, feeds: Vec<String>) -> Result<(), HttpError> {
        match self
            .write
            .send(ws_feed_msg(feeds, SubscribeAction::Unsubscribe))
            .await
        {
            Ok(()) => Ok(()),
            Err(e) => Err(HttpError::TransportError(e.to_string())),
        }
    }
}

fn ws_feed_msg(feeds: Vec<String>, action: SubscribeAction) -> Message {
    let msg_type = action.into();
    Message::Text(serde_json::to_string(&SubscribeMsg { msg_type, feeds }).unwrap())
}
