use futures_util::{
    stream::{SplitSink, SplitStream},
    Future, SinkExt, StreamExt, TryFutureExt,
};
use serde_derive::Deserialize;
use std::{
    env,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::http::HttpError;

use self::message::{SubscribeAction, SubscribeMsg};

mod auth;
pub mod message;

static DEFAULT_WS_SERVER_URL: &str = "wss://ws.sfox.com/ws";

#[derive(Clone, Error, Debug, Deserialize)]
pub enum WebsocketClientError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("could not create http client: {0}")]
    InitializationError(String),
    #[error("could not lock the write stream {0}")]
    LockError(String),
    #[error("could not send message: {0}")]
    TxError(String),
}

#[derive(Debug)]
pub struct Client {
    pub server_url: String,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
}

impl Client {
    /// Create a new server with the URL set in the environment
    pub fn new() -> impl Future<Output = Result<Client, WebsocketClientError>> {
        let server_url =
            env::var("SFOX_WS_SERVER_URL").unwrap_or_else(|_| DEFAULT_WS_SERVER_URL.to_string());

        Client::new_with_server_url(server_url)
    }

    /// Create a new server with the given server URL; useful for testing.
    pub fn new_with_server_url(
        server_url: String,
    ) -> impl Future<Output = Result<Client, WebsocketClientError>> {
        connect_async(server_url.clone())
            .map_err(|e| {
                WebsocketClientError::InitializationError(format!(
                    "Could not connect to websocket server: {}",
                    e
                ))
            })
            .and_then(|socket| async move {
                let (stream, response) = socket;

                if !response.status().is_informational() {
                    return Err(WebsocketClientError::InitializationError(format!(
                        "Websocket connection unsuccessful: {:?}",
                        response
                    )));
                }

                let (write, read) = stream.split();

                Ok(Client {
                    server_url,
                    read,
                    write: Arc::new(Mutex::new(write)),
                })
            })
    }

    /// Subscribe to the given feeds.
    pub async fn subscribe(&self, feeds: Vec<String>) -> Result<(), HttpError> {
        match self.send(feed_msg(feeds, SubscribeAction::Subscribe)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::TransportError(e.to_string())),
        }
    }

    // Unsubscribe from feeds that this socket has previously subscribed to.
    pub async fn unsubscribe(&self, feeds: Vec<String>) -> Result<(), HttpError> {
        match self
            .send(feed_msg(feeds, SubscribeAction::Unsubscribe))
            .await
        {
            Ok(()) => Ok(()),
            Err(e) => Err(HttpError::TransportError(e.to_string())),
        }
    }

    async fn send(&self, msg: Message) -> Result<(), WebsocketClientError> {
        match self.write.lock() {
            Ok(mut write) => write.send(msg).await.map_err(|e| {
                WebsocketClientError::TxError(format!("Could not send message: {}", e))
            }),
            Err(e) => Err(WebsocketClientError::TxError(e.to_string())),
        }
    }
}

fn feed_msg(feeds: Vec<String>, action: SubscribeAction) -> Message {
    let msg_type = action.into();
    Message::Text(serde_json::to_string(&SubscribeMsg { msg_type, feeds }).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::util::server::{start_test_ws_server, stop_test_ws_server};

    use super::*;

    #[tokio::test]
    async fn test_subscribe() {
        let (stop, addr, _handle) = start_test_ws_server().await;

        let client = Client::new_with_server_url(format!("ws://{}", addr))
            .await
            .unwrap();

        let result = client.subscribe(vec!["btcusd".into()]).await;

        stop_test_ws_server(stop).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let (stop, addr, _handle) = start_test_ws_server().await;
        let client = Client::new_with_server_url(format!("ws://{}", addr))
            .await
            .unwrap();

        let result = client.unsubscribe(vec!["orders".into()]).await;

        stop_test_ws_server(stop).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_feed_msg() {
        let feeds = vec!["btcusd".into(), "ethusd".into()];
        let msg = feed_msg(feeds, SubscribeAction::Subscribe);
        assert!(
            msg == Message::Text(r#"{"type":"subscribe","feeds":["btcusd","ethusd"]}"#.to_string())
        );
    }

    #[tokio::test]
    async fn test_new() {
        let maybe_ws = Client::new().await;
        assert!(maybe_ws.is_ok());
    }
}
