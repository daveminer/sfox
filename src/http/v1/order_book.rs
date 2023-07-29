use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::SFox;
use crate::http::{HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct OrderBook {
    pub pair: String,
    pub currency: Option<String>,
    pub asks: Vec<OpenOrder>,
    pub bids: Vec<OpenOrder>,
    pub market_making: MarketMaking,
    //pub timestamps: HashMap<String, Vec<usize>>,
    pub lastupdated: usize,
    pub lastpublished: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarketMaking {
    pub asks: Vec<OpenOrder>,
    pub bids: Vec<OpenOrder>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenOrder {
    pub price: f64,
    pub volume: f64,
    pub exchange: String,
}

impl SFox {
    pub fn order_book(self, pair: &str) -> impl Future<Output = Result<OrderBook, HttpError>> {
        let query_str = format!("markets/orderbook/{}", pair);
        let url = self.url_for_v1_resource(&query_str);
        //println!("URL: {}", url);

        self.request(HttpVerb::Get, &url, None)
    }
}
