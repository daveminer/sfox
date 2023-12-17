//! SFox API client library.
//!
//! This library provides an asynchronous client for the [SFox API](https://docs.sfox.com/).
//!
//! # Setup
//!
//! The `SFOX_API_KEY` environment variable is required. API keys can be obtained from the [dashboard](https://trade.sfox.com/account/api).
//!
//! `SFOX_HTTP_SERVER_URL` and `SFOX_WS_SERVER_URL` environment variables are optional. If not set, the default values are used.
//! # Example
//!
//! ```
//! let sfox = http::new().unwrap();
//! let order_book: sfox::http::v1::order_book::OrderBook = sfox.order_book("btcusd").await.unwrap();
//! println!("First order book bid is: #{}", order_book.bids[0]);
//! ```

/// Models the resources of the SFox HTTP API.
pub mod http;
pub mod websocket;

/// Utility functions for testing.
pub(crate) mod util;
