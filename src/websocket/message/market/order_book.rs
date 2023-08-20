use serde::Deserialize;

use crate::websocket::message::WsResponse;

pub type WsOrderBookResponse = WsResponse<WsOrderBookResponsePayload>;

#[derive(Debug, Deserialize)]
pub struct WsOrderBookResponsePayload {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub lastpublished: usize,
    pub lastupdated: usize,
    pub market_making: MarketMaking,
    pub pair: String,
}

#[derive(Debug, Deserialize)]
pub struct MarketMaking {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub source: String,
}

pub enum BookType {
    FeeAdjusted,
    Unadjusted,
}

impl ToString for BookType {
    fn to_string(&self) -> String {
        match self {
            BookType::FeeAdjusted => "net".to_string(),
            BookType::Unadjusted => "sfox".to_string(),
        }
    }
}

pub fn order_book_feed(basequote: &str, book_type: BookType) -> String {
    format!("orderbook.{}.{}", book_type.to_string(), basequote)
}
