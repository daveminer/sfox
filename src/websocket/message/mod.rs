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

pub mod account;
pub mod market;

pub type BalancesResponse = WsResponse<Vec<BalancePayload>>;
pub type OrderResponse = WsResponse<Vec<OrderPayload>>;
pub type PostTradeSettlemtnResponse = WsResponse<PostTradeSettlementPayload>;
pub type OrderbookResponse = WsResponse<Orderbook>;
pub type TickerResponse = WsResponse<Ticker>;
pub type TradeResponse = WsResponse<Trade>;

/// Websocket messages fall under one of these categories.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Feed {
    Balances,
    Orders,
    PostTradeSettlement,
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
///     feed_type: Feed::RawOrderbook,
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
            None => {
                return Err(WebsocketClientError::ParseError(
                    "could not find 'type' key in message".to_string(),
                ))
            }
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
        util::fixtures,
        websocket::{
            message::{
                BalancesResponse, Feed, OrderResponse, OrderbookResponse, TickerResponse,
                TradeResponse,
            },
            Client,
        },
    };

    #[tokio::test]
    async fn test_feed_message_type_err() {
        let msg = Message::Text(fixtures::SUBSCRIBE_PAYLOAD.to_string());
        let feed_msg_type = Client::feed_message_type(msg);

        assert!(feed_msg_type.is_err());
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
}
