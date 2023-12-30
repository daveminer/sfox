use serde_derive::Deserialize;
use std::convert::TryFrom;

use self::{order_book::WsOrderBook, ticker::WsTicker, trade::WsTrade};

use super::WsResponse;

pub mod order_book;
pub mod ticker;
pub mod trade;

#[derive(Debug, Deserialize, PartialEq)]
pub enum WsMarket {
    OrderBook(WsOrderBook),
    Ticker(WsTicker),
    Trade(WsTrade),
}

impl TryFrom<WsResponse<WsMarket>> for WsResponse<WsOrderBook> {
    type Error = &'static str;

    fn try_from(value: WsResponse<WsMarket>) -> Result<Self, Self::Error> {
        if let WsMarket::OrderBook(payload) = value.payload {
            Ok(WsResponse {
                recipient: value.recipient,
                payload,
                sequence: value.sequence,
                timestamp: value.timestamp,
            })
        } else {
            Err("Cannot convert WsResponse<WsMarket> into WsResponse<WsOrderBook>")
        }
    }
}

impl TryFrom<WsResponse<WsMarket>> for WsResponse<WsTicker> {
    type Error = &'static str;

    fn try_from(value: WsResponse<WsMarket>) -> Result<Self, Self::Error> {
        if let WsMarket::Ticker(payload) = value.payload {
            Ok(WsResponse {
                recipient: value.recipient,
                payload,
                sequence: value.sequence,
                timestamp: value.timestamp,
            })
        } else {
            Err("Cannot convert WsResponse<WsMarket> into WsResponse<WsTicker>")
        }
    }
}

impl TryFrom<WsResponse<WsMarket>> for WsResponse<WsTrade> {
    type Error = &'static str;

    fn try_from(value: WsResponse<WsMarket>) -> Result<Self, Self::Error> {
        if let WsMarket::Trade(payload) = value.payload {
            Ok(WsResponse {
                recipient: value.recipient,
                payload,
                sequence: value.sequence,
                timestamp: value.timestamp,
            })
        } else {
            Err("Cannot convert WsResponse<WsMarket> into WsResponse<WsTrade>")
        }
    }
}

pub fn is_orderbook_msg(msg: WsMarket) -> bool {
    match msg {
        WsMarket::OrderBook(_) => true,
        _ => false,
    }
}

pub fn is_ticker_msg(msg: WsMarket) -> bool {
    match msg {
        WsMarket::Ticker(_) => true,
        _ => false,
    }
}
pub fn is_trades_msg(msg: WsMarket) -> bool {
    match msg {
        WsMarket::Trade(_) => true,
        _ => false,
    }
}
