//! SFox API client library.
//!
//! This library provides an asynchronous client for the [SFox API](https://docs.sfox.com/) ove HTTP and WebSockets.
//! FIX is not supported.
//!
//! # Setup
//!
//! The `SFOX_API_KEY` environment variable is required. API keys can be obtained from the [dashboard](https://trade.sfox.com/account/api).
//!
//! `SFOX_HTTP_SERVER_URL` and `SFOX_WS_SERVER_URL` environment variables are optional. If not set, the default values are used.

/// Models the resources of the SFox HTTP API.
pub mod http;
/// Offers convenience methods for session management and types for message deserialization.
pub mod websocket;

/// Test helpers.
pub(crate) mod util;
