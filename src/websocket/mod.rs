use futures_util::{stream::SplitSink, Future, SinkExt, TryFutureExt};
use serde_derive::Deserialize;
use std::env;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use self::message::{Feed, SubscribeAction, SubscribeMsg};

pub mod auth;
pub mod message;

type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

static DEFAULT_WS_SERVER_URL: &str = "wss://ws.sfox.com/ws";

#[derive(Clone, Error, Debug, Deserialize)]
pub enum WebsocketClientError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("could not create http client: {0}")]
    InitializationError(String),
    #[error("could not lock the write stream {0}")]
    LockError(String),
    #[error("could not parse: {0}")]
    ParseError(String),
    #[error("could not send message: {0}")]
    TxError(String),
}

#[derive(Debug)]
pub struct Client {
    pub server_url: String,
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    /// Create a new server with the URL set in the environment.
    pub fn new() -> impl Future<Output = Result<Client, WebsocketClientError>> {
        let server_url =
            env::var("SFOX_WS_SERVER_URL").unwrap_or_else(|_| DEFAULT_WS_SERVER_URL.to_string());

        Client::new_with_server_url(server_url)
    }

    /// Create a new server with the given server URL; used for testing.
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

                Ok(Client { server_url, stream })
            })
    }

    /// Subscribe to the provided feeds.
    pub async fn subscribe(
        write: &mut WsSink,
        feed_type: Feed,
        feeds: Vec<String>,
    ) -> Result<(), WebsocketClientError> {
        Self::send(
            write,
            feed_msg(feeds, feed_type, SubscribeAction::Subscribe),
        )
        .await
    }

    /// Unsubscribe from feeds that this socket has previously subscribed to.
    pub async fn unsubscribe(
        write: &mut WsSink,
        feed_type: Feed,
        feeds: Vec<String>,
    ) -> Result<(), WebsocketClientError> {
        Self::send(
            write,
            feed_msg(feeds, feed_type, SubscribeAction::Unsubscribe),
        )
        .await
    }

    async fn send(write: &mut WsSink, msg: Message) -> Result<(), WebsocketClientError> {
        write
            .send(msg)
            .await
            .map_err(|e| WebsocketClientError::TxError(format!("Could not send message: {}", e)))
    }
}

fn feed_msg(feeds: Vec<String>, feed_type: Feed, action: SubscribeAction) -> Message {
    Message::Text(
        serde_json::to_string(&SubscribeMsg {
            action: action.into(),
            feed_type,
            feeds,
        })
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;

    use super::*;
    use crate::util::server::{start_test_ws_server, stop_test_ws_server};

    #[tokio::test]
    async fn test_subscribe() {
        let (stop, addr, _handle) = start_test_ws_server().await;

        let client = Client::new_with_server_url(format!("ws://{}", addr))
            .await
            .unwrap();

        let (mut write, _read) = client.stream.split();
        let ticker_result =
            Client::subscribe(&mut write, Feed::Ticker, vec!["btcusd".into()]).await;
        let trade_result = Client::subscribe(&mut write, Feed::Trade, vec!["btcusd".into()]).await;

        stop_test_ws_server(stop).await;
        assert!(ticker_result.is_ok());
        assert!(trade_result.is_ok());
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let (stop, addr, _handle) = start_test_ws_server().await;
        let client = Client::new_with_server_url(format!("ws://{}", addr))
            .await
            .unwrap();

        let (mut write, _read) = client.stream.split();
        let result =
            Client::unsubscribe(&mut write, Feed::NetOrderbook, vec!["orders".into()]).await;

        stop_test_ws_server(stop).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new() {
        let maybe_ws = Client::new().await;
        assert!(maybe_ws.is_ok());
    }
}
