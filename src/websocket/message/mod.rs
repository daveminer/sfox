use serde::de::value::Error;
use serde::de::{DeserializeOwned, Error as DeError};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use self::account::balance::BalancePayload;
use self::account::order::OrderPayload;
use self::account::post_trade_settlement::PostTradeSettlementPayload;
use self::market::orderbook::Orderbook;
use self::market::ticker::Ticker;
use self::market::trade::Trade;

use super::{Client, WebsocketClientError};

/// Types and subscription builders for balances, orders, and post-trade settlement.
pub mod account;
/// Types and subscription builders for orderbook, ticker, and trade.
pub mod market;

pub type BalancesResponse = WsResponse<Vec<BalancePayload>>;
pub type OrderResponse = WsResponse<Vec<OrderPayload>>;
pub type PostTradeSettlementResponse = WsResponse<PostTradeSettlementPayload>;
pub type OrderbookResponse = WsResponse<Orderbook>;
pub type TickerResponse = WsResponse<Ticker>;
pub type TradeResponse = WsResponse<Trade>;

/// Converts a JSON value from deserialized websocket message into
/// a typed struct, if possible.
trait FromJson {
    fn from_json(value: Value) -> Result<Self, Error>
    where
        Self: Sized;
}

/// Websocket messages fall under one of these categories.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Feed {
    Balances,
    Orders,
    PostTradeSettlement,
    NetOrderbook,
    RawOrderbook,
    System,
    Ticker,
    Trade,
}

/// The outer shape of a message received from an active subscription.
#[derive(Debug, Deserialize, PartialEq)]
pub struct WsResponse<T> {
    pub recipient: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

impl<T> FromJson for WsResponse<T>
where
    T: DeserializeOwned,
{
    fn from_json(value: Value) -> Result<Self, Error> {
        let recipient = match value.get("recipient").and_then(Value::as_str) {
            Some(recipient) => recipient,
            None => {
                return Err(Error::custom(
                    "could not find 'recipient' key in message".to_string(),
                ))
            }
        };

        let payload: T = match value.get("payload") {
            Some(payload) => serde_json::from_value(payload.clone()).unwrap(),
            None => {
                return Err(Error::custom(
                    "could not find 'payload' key in message".to_string(),
                ))
            }
        };

        let sequence = match value.get("sequence").and_then(Value::as_u64) {
            Some(sequence) => sequence as usize,
            None => {
                return Err(Error::custom(
                    "could not find 'sequence' key in message".to_string(),
                ))
            }
        };

        let timestamp = match value.get("timestamp").and_then(Value::as_u64) {
            Some(timestamp) => timestamp as usize,
            None => {
                return Err(Error::custom(
                    "could not 'timestamp' key in message".to_string(),
                ))
            }
        };

        Ok(WsResponse {
            recipient: recipient.to_string(),
            payload,
            sequence,
            timestamp,
        })
    }
}

/// Response to a system-related websocket message.
#[derive(Debug, Deserialize)]
pub struct WsSystemResponse<T> {
    #[serde(rename = "type")]
    pub message_type: String,
    pub payload: T,
    pub sequence: usize,
    pub timestamp: usize,
}

impl<T> FromJson for WsSystemResponse<T>
where
    T: DeserializeOwned,
{
    fn from_json(value: Value) -> Result<Self, Error> {
        let message_type = match value.get("type").and_then(Value::as_str) {
            Some(message_type) => message_type,
            None => {
                return Err(Error::custom(
                    "could not find 'type' key in message".to_string(),
                ))
            }
        };

        let payload: T = match value.get("payload") {
            Some(payload) => serde_json::from_value(payload.clone()).unwrap(),
            None => {
                return Err(Error::custom(
                    "could not find 'payload' key in message".to_string(),
                ))
            }
        };

        let sequence = match value.get("sequence").and_then(Value::as_u64) {
            Some(sequence) => sequence as usize,
            None => {
                return Err(Error::custom(
                    "could not find 'sequence' key in message".to_string(),
                ))
            }
        };

        let timestamp = match value.get("timestamp").and_then(Value::as_u64) {
            Some(timestamp) => timestamp as usize,
            None => {
                return Err(Error::custom(
                    "could not 'timestamp' key in message".to_string(),
                ))
            }
        };

        Ok(WsSystemResponse {
            message_type: message_type.to_string(),
            payload,
            sequence,
            timestamp,
        })
    }
}

///
/// A message sent to the websocket server. Contains the action to be taken, the type of feed, and
/// the feeds to subscribe to.
///
/// # Example
/// ```
/// use sfox::websocket::{message::Feed, message::SubscribeMsg};
///
/// let order_msg = SubscribeMsg {
///     action: "subscribe".to_string(),
///     feed_type: Feed::RawOrderbook,
///     feeds: vec!["btcusd".to_string()],
/// };
/// assert_eq!(
///     serde_json::to_string(&order_msg).unwrap(),
///     "{\"type\":\"subscribe\",\"feeds\":[\"orderbook.sfox.btcusd\"]}"
/// );
/// ```
///
#[derive(Debug, Deserialize)]
pub struct SubscribeMsg {
    pub action: String,
    #[serde(rename = "type")]
    pub feed_type: Feed,
    pub feeds: Vec<String>,
}

impl Serialize for SubscribeMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let prefix_or_msg = match self.feed_type {
            Feed::Balances => "private.user.balances",
            Feed::NetOrderbook => "orderbook.net",
            Feed::Orders => "private.user.open-orders",
            Feed::PostTradeSettlement => "private.user.post-trade-settlement",
            Feed::RawOrderbook => "orderbook.sfox",
            Feed::System => "system",
            Feed::Ticker => "ticker.sfox",
            Feed::Trade => "trades.sfox",
        };

        let feeds: Vec<String> = if self.feed_type == Feed::NetOrderbook
            || self.feed_type == Feed::RawOrderbook
            || self.feed_type == Feed::Ticker
            || self.feed_type == Feed::Trade
        {
            self.feeds
                .iter()
                .map(|feed| format!("{}.{}", prefix_or_msg, feed))
                .collect()
        } else {
            vec![prefix_or_msg.into()]
        };

        let mut state = serializer.serialize_struct("SubscribeMsg", 3)?;
        state.serialize_field("type", &self.action)?;
        state.serialize_field("feeds", &feeds)?;
        state.end()
    }
}

impl Client {
    /// Given a websocket message, determine the type of feed it is. This can be
    /// used for deserialization in a message handler.
    pub fn feed_message_type(message: Message) -> Result<Feed, WebsocketClientError> {
        let message = match message.to_text() {
            Ok(message) => message,
            Err(e) => {
                return Err(WebsocketClientError::ParseError(format!(
                    "Not a message with text: {}",
                    e
                )))
            }
        };

        let msg_json = match serde_json::from_str::<Value>(message) {
            Ok(json) => json,
            Err(e) => {
                return Err(WebsocketClientError::ParseError(format!(
                    "could not parse json: {}",
                    e
                )))
            }
        };

        let recipient = match msg_json.get("recipient").and_then(Value::as_str) {
            Some(recipient) => recipient,
            None => match msg_json.get("type").and_then(Value::as_str) {
                Some(_msg_type) => return Ok(Feed::System),
                None => {
                    return Err(WebsocketClientError::ParseError(
                        "could not find a matching key in message".to_string(),
                    ))
                }
            },
        };

        let msg_type = match Self::identify_recipient(recipient) {
            Some(msg_type) => msg_type,
            None => {
                return Err(WebsocketClientError::ParseError(format!(
                    "unknown feed type of {}",
                    recipient
                )))
            }
        };

        Ok(msg_type)
    }

    fn identify_recipient(recipient: &str) -> Option<Feed> {
        if recipient.starts_with("orderbook.net") {
            Some(Feed::NetOrderbook)
        } else if recipient.starts_with("orderbook.sfox") {
            Some(Feed::RawOrderbook)
        } else if recipient.starts_with("ticker") {
            Some(Feed::Ticker)
        } else if recipient.starts_with("trades") {
            Some(Feed::Trade)
        } else if recipient.starts_with("private.user.balances") {
            Some(Feed::Balances)
        } else if recipient.starts_with("private.user.open-orders") {
            Some(Feed::Orders)
        } else if recipient.starts_with("private.user.post-trade-settlement") {
            Some(Feed::PostTradeSettlement)
        } else {
            None
        }
    }
}

/// Subscribe / Unsubscribe
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
    use serde_json::{json, Value};
    use tokio_tungstenite::tungstenite::Message;

    use crate::{
        util::fixtures,
        websocket::{
            message::{
                BalancesResponse, Feed, FromJson, OrderResponse, OrderbookResponse, TickerResponse,
                TradeResponse, WsResponse, WsSystemResponse,
            },
            Client,
        },
    };

    #[tokio::test]
    async fn test_feed_message_type_err() {
        let msg = Message::Text("{}".to_string());
        let feed_msg_type = Client::feed_message_type(msg);

        assert!(feed_msg_type.is_err());
    }

    #[tokio::test]
    async fn test_feed_message_type_system() {
        let msg = Message::Text(fixtures::SUBSCRIBE_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::System);
    }

    #[tokio::test]
    async fn test_feed_message_type_orderbook() {
        let msg = Message::Text(fixtures::NET_ORDERBOOK_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::NetOrderbook);
    }

    #[tokio::test]
    async fn test_feed_message_type_ticker() {
        let msg = Message::Text(fixtures::TICKER_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::Ticker);
    }

    #[tokio::test]
    async fn test_feed_message_type_trade() {
        let msg = Message::Text(fixtures::TRADE_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::Trade);
    }

    #[tokio::test]
    async fn test_feed_message_type_open_orders() {
        let msg = Message::Text(fixtures::OPEN_ORDERS_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::Orders);
    }

    #[tokio::test]
    async fn test_feed_message_type_balances() {
        let msg = Message::Text(fixtures::BALANCES_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::Balances);
    }

    #[tokio::test]
    async fn test_feed_message_type_post_trade_settlement() {
        let msg = Message::Text(fixtures::POST_TRADE_SETTLEMENT_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg).unwrap();

        assert!(feed_msg_type == Feed::PostTradeSettlement);
    }

    #[tokio::test]
    async fn test_deserialize_balance() {
        let balances_payload = fixtures::BALANCES_PAYLOAD;

        let _balances_response: BalancesResponse = serde_json::from_str(balances_payload).unwrap();
    }

    #[tokio::test]
    async fn test_serialize_balance() {
        let balance_subscription =
            fixtures::subscribe_msg("subscribe".into(), Feed::Balances, vec!["btcusd".into()]);

        let msg = serde_json::to_string(&balance_subscription).unwrap();

        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"private.user.balances\"]}");
    }

    #[tokio::test]
    async fn test_deserialize_open_orders() {
        let open_orders_payload = fixtures::OPEN_ORDERS_PAYLOAD;

        let _open_orders_response: OrderResponse =
            serde_json::from_str(open_orders_payload).unwrap();
    }

    #[tokio::test]
    async fn test_serialize_open_orders() {
        let balance_subscription =
            fixtures::subscribe_msg("subscribe".into(), Feed::Orders, vec![]);

        let msg = serde_json::to_string(&balance_subscription).unwrap();
        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"private.user.open-orders\"]}");
    }

    #[tokio::test]
    async fn test_deserialize_orders() {
        let order_payload = fixtures::NET_ORDERBOOK_PAYLOAD;

        let _order_response: OrderbookResponse = serde_json::from_str(order_payload).unwrap();
    }

    #[tokio::test]
    async fn test_serialize_net_orders() {
        let balance_subscription = fixtures::subscribe_msg(
            "subscribe".into(),
            Feed::NetOrderbook,
            vec!["btcusd".into(), "ethusd".into()],
        );

        let msg = serde_json::to_string(&balance_subscription).unwrap();
        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"orderbook.net.btcusd\",\"orderbook.net.ethusd\"]}");
    }

    #[tokio::test]
    async fn test_serialize_raw_orders() {
        let balance_subscription = fixtures::subscribe_msg(
            "subscribe".into(),
            Feed::RawOrderbook,
            vec!["btcusd".into(), "ethusd".into()],
        );

        let msg = serde_json::to_string(&balance_subscription).unwrap();
        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"orderbook.sfox.btcusd\",\"orderbook.sfox.ethusd\"]}");
    }

    #[tokio::test]
    async fn test_deserialize_tickers() {
        let ticker = fixtures::TICKER_PAYLOAD;

        let _ticker_response: TickerResponse = serde_json::from_str(ticker).unwrap();
    }

    #[tokio::test]
    async fn test_serialize_tickers() {
        let balance_subscription = fixtures::subscribe_msg(
            "subscribe".into(),
            Feed::Ticker,
            vec!["btcusd".into(), "ethusd".into()],
        );

        let msg = serde_json::to_string(&balance_subscription).unwrap();
        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"ticker.sfox.btcusd\",\"ticker.sfox.ethusd\"]}");
    }

    #[tokio::test]
    async fn test_deserialize_trade() {
        let trade = fixtures::TRADE_PAYLOAD;

        let _trade_response: TradeResponse = serde_json::from_str(trade).unwrap();
    }

    #[tokio::test]
    async fn test_serialize_trades() {
        let balance_subscription = fixtures::subscribe_msg(
            "subscribe".into(),
            Feed::Trade,
            vec!["btcusd".into(), "ethusd".into()],
        );

        let msg = serde_json::to_string(&balance_subscription).unwrap();
        assert!(msg == "{\"type\":\"subscribe\",\"feeds\":[\"trades.sfox.btcusd\",\"trades.sfox.ethusd\"]}");
    }

    #[tokio::test]
    async fn test_deserialize_system_response() {
        let system_response_payload = r#"
    {
        "type": "system_type",
        "payload": { "some_field": "some_value" },
        "sequence": 123,
        "timestamp": 456
    }
    "#;

        let _system_response: WsSystemResponse<Value> =
            WsSystemResponse::from_json(serde_json::from_str(system_response_payload).unwrap())
                .unwrap();
    }

    #[test]
    fn test_deserialize_ws_response() {
        let ws_response_payload = json!({
            "recipient": "private.user.balances",
            "payload": { "some_field": "some_value" },
            "sequence": 123,
            "timestamp": 456
        });

        let ws_response: Result<WsResponse<Value>, _> = WsResponse::from_json(ws_response_payload);

        assert!(ws_response.is_ok());
        let ws_response = ws_response.unwrap();

        assert_eq!(ws_response.recipient, "private.user.balances");
        assert_eq!(ws_response.sequence, 123);
        assert_eq!(ws_response.timestamp, 456);

        let payload = ws_response.payload.as_object().unwrap();
        assert_eq!(payload.get("some_field").unwrap(), "some_value");
    }
}
