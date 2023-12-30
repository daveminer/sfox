use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Orderbook {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub lastpublished: usize,
    pub lastupdated: usize,
    pub market_making: MarketMaking,
    pub pair: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MarketMaking {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(from = "(f64, f64, String)")]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub source: String,
}

impl From<(f64, f64, String)> for Order {
    fn from(data: (f64, f64, String)) -> Self {
        Order {
            price: data.0,
            quantity: data.1,
            source: data.2,
        }
    }
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
