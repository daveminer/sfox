//! Unofficial Rust client for the SFox API.
//!
//! This library provides an asynchronous, tokio-based client for the [SFox API](https://docs.sfox.com/) over HTTP and WebSockets.
//! FIX is not supported.
//!
//! # Setup
//!
//! The `SFOX_API_KEY` environment variable is required. API keys can be obtained from the [dashboard](https://trade.sfox.com/account/api).
//!
//! `SFOX_HTTP_SERVER_URL` and `SFOX_WS_SERVER_URL` environment variables are optional. If not set, the default values are used.
//!
//! # Usage
//!
//! Both the HTTP and WebSocket APIs are modeled as asynchronous clients. The HTTP client is used for all REST API calls,
//! while the WebSocket client is used for subscribing to market data feeds. Both clients are intended to have a lifetime
//! across the program but the HTTP client is threadsafe and can be used ephermerally. The websocket client should be
//! reused as it maintains a connection to the server. Note that the socket must be explicitly authenticated after
//! instantiation.
//!
//! ## HTTP Client
//!
//! ```no_run
//! use sfox::http::v1::currency::CurrencyPair;
//! use sfox::http::Client;
//! use std::collections::HashMap;
//!
//! tokio_test::block_on(async {
//!   let http_client = Client::new().unwrap();
//!   let _pairs: HashMap<String, CurrencyPair> = http_client.currency_pairs().await.unwrap();
//! });
//! ```
//!
//! ## WebSocket Client
//!
//! ```no_run
//! use futures::StreamExt;
//! use sfox::websocket::{message::Feed, Client};
//!
//! tokio_test::block_on(async {
//!   let sfox_ws = sfox::websocket::Client::new().await.unwrap();
//!   let (mut write, mut read) = sfox_ws.stream.split();
//!
//! // Start a task to read messages from the SFox stream
//!   let _sfox_handle = tokio::spawn(async move {
//!       while let Some(message) = read.next().await {
//!           println!("Received message: {:?}", message);
//!       }
//!   });
//!
//!   // Subscribe to a feed on the websocket server
//!   let _ticker_subscription = Client::subscribe(&mut write, Feed::Ticker, vec!["btcusd".to_string()]).await;
//!
//!   // Authenticate in order to access private feeds
//!   let _authentication_attempt = Client::authenticate(&mut write).await;
//!
//!   // Subscribe to a private feed
//!   let _balance_subscription = Client::subscribe(&mut write, Feed::Balances, vec![]).await;
//! });
//! ```

/// Models the resources of the SFox HTTP API with [tokio](https://crates.io/crates/tokio)-based convenience methods for making HTTP requests to the SFOX API.
pub mod http;
/// Offers convenience methods for authentication and feed subscription, as well as types for message deserialization.
pub mod websocket;

/// Test helpers.
pub(crate) mod util;
