use serde::de::DeserializeOwned;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use self::market::WsMarket;

use super::{Client, WebsocketClientError};

pub mod account;
pub mod market;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FeedType {
    NetOrderbook,
    RawOrderbook,
    Ticker,
    Trade,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WsResponse<T> {
    pub recipient: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
pub struct WsSystemResponse<T> {
    #[serde(rename = "type")]
    pub message_type: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

///
/// A message sent to the websocket server. Contains the action to be taken, the type of feed, and
/// the feeds to subscribe to.
///
/// # Example
/// ```
/// use sfox::websocket::{FeedType, message::SubscribeMsg};
///
/// let order_msg = SubscribeMsg {
///     action: "subscribe".to_string(),
///     feed_type: FeedType::RawOrderbook,
///     feeds: vec!["btcusd".to_string()],
/// };
/// assert_eq!(
///     serde_json::to_string(&order_msg).unwrap(),
///     "{\"action\":\"subscribe\",\"type\":\"RawOrderbook\",\"feeds\":[\"orderbook.sfox.btcusd\"]}"
/// );
/// ```
///
#[derive(Debug, Deserialize)]
pub struct SubscribeMsg {
    pub action: String,
    #[serde(rename = "type")]
    pub feed_type: FeedType,
    pub feeds: Vec<String>,
}

impl Serialize for SubscribeMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let prefix = match self.feed_type {
            FeedType::NetOrderbook => "orderbook.net",
            FeedType::RawOrderbook => "orderbook.sfox",
            FeedType::Ticker => "ticker.sfox",
            FeedType::Trade => "trades.sfox",
        };

        let feeds: Vec<String> = self
            .feeds
            .iter()
            .map(|feed| format!("{}.{}", prefix, feed))
            .collect();

        let mut state = serializer.serialize_struct("SubscribeMsg", 3)?;
        state.serialize_field("action", &self.action)?;
        state.serialize_field("type", &self.feed_type)?;
        state.serialize_field("feeds", &feeds)?;
        state.end()
    }
}

impl Client {
    // pub fn feed_message_type(
    //     message: Message,
    // ) -> Result<WsMarketResponsePayload, WebsocketClientError> {
    //     let message = match message.to_text() {
    //         Ok(message) => message,
    //         Err(e) => {
    //             return Err(WebsocketClientError::ParseError(format!(
    //                 "Not a message with text: {}",
    //                 e
    //             )))
    //         }
    //     };

    //     let err_msg: String = match serde_json::from_str::<Value>(message) {
    //         Ok(json) => {
    //             if let Some(recipient) = json.get("recipient").and_then(Value::as_str) {
    //                 if recipient.starts_with("orderbook.net") {
    //                     return Ok(WsOrderBookResponsePayload);
    //                 } else if recipient.starts_with("orderbook.sfox") {
    //                     return Ok(FeedType::RawOrderbook);
    //                 } else if recipient.starts_with("ticker") {
    //                     return Ok(FeedType::Ticker);
    //                 } else if recipient.starts_with("trades") {
    //                     return Ok(FeedType::Trade);
    //                 } else {
    //                     format!("unknown feed type of {}", recipient)
    //                 }
    //             } else {
    //                 format!("'recipient' key not found: {}", json)
    //             }
    //         }
    //         Err(e) => format!("could not parse json: {}", e),
    //     };

    //     return Err(WebsocketClientError::ParseError(err_msg.to_string()));
    // }

    pub fn parse_feed_message(
        message: Message,
    ) -> Result<WsResponse<WsMarket>, WebsocketClientError> {
        let text = match message.to_text() {
            Ok(text) => text,
            Err(e) => {
                return Err(WebsocketClientError::ParseError(format!(
                    "Not a message with text: {}",
                    e
                )))
            }
        };

        let value: Value = serde_json::from_str(text).map_err(|e| {
            WebsocketClientError::ParseError(format!("Unable to parse to JSON: {}", e))
        })?;

        // let recipient = value
        //     .get("recipient")
        //     .and_then(Value::as_str)
        //     .ok_or_else(|| {
        //         WebsocketClientError::ParseError(format!("'recipient' key not found: {}", value))
        //     })?;

        serde_json::from_value::<WsResponse<WsMarket>>(value)
            .map_err(|e| WebsocketClientError::ParseError(format!("Invalid payload: {:?}", e)))
    }
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

#[cfg(test)]
mod tests {
    use tokio_tungstenite::tungstenite::Message;

    use crate::{
        http::v1::order_book::OrderBook,
        util::fixtures,
        websocket::{
            message::{
                market::{order_book::WsOrderBook, WsMarket},
                WsResponse,
            },
            Client,
        },
    };

    // #[tokio::test]
    // async fn test_feed_message_type_err() {
    //     let msg = Message::Text(fixtures::WS_SUBSCRIBE_RESPONSE.to_string());
    //     let feed_msg_type = Client::feed_message_type(msg);

    //     assert!(feed_msg_type.is_err());
    // }

    // #[tokio::test]
    // async fn test_feed_message_type_order() {
    //     let msg = Message::Text(fixtures::WS_NET_ORDERBOOK_RESPONSE.to_string());
    //     let feed_msg_type = Client::feed_message_type(msg).unwrap();

    //     assert!(
    //         feed_msg_type
    //             == WsMarketResponsePayload::WsOrderBookResponsePayload(
    //                 fixtures::WS_NET_ORDERBOOK_RESPONSE_PAYLOAD.clone()
    //             )
    //     );
    // }

    // #[tokio::test]
    // async fn test_feed_message_type_ticker() {
    //     let msg = Message::Text(fixtures::WS_TICKER_RESPONSE.to_string());
    //     let feed_msg_type = Client::feed_message_type(msg).unwrap();

    //     assert!(feed_msg_type == FeedType::Ticker);
    // }

    // #[tokio::test]
    // async fn test_feed_message_type_trade() {
    //     let msg = Message::Text(fixtures::WS_TRADES_RESPONSE.to_string());
    //     let feed_msg_type = Client::feed_message_type(msg).unwrap();

    //     assert!(feed_msg_type == FeedType::Trade);
    // }

    #[tokio::test]
    async fn test_parse_feed_message() {
        let msg = Message::Text(fixtures::WS_NET_ORDERBOOK_RESPONSE.to_string());
        println!("MSG: {:?}", msg);
        let feed_msg_type: WsResponse<OrderBook> = Client::parse_feed_message(msg).unwrap();
        let ob: WsResponse<WsOrderBook> = feed_msg_type.try_into().unwrap();
        //let ob = WsOrderBook::try_from(feed_msg_type.payload);

        let expected: WsResponse<WsOrderBook> =
            serde_json::from_str(fixtures::WS_NET_ORDERBOOK_RESPONSE.clone()).unwrap();

        assert!(ob == expected);
    }
}
