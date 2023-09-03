/// HTTP client for interacting with the SFox API.
pub mod http;
/// Websocket client with convenience methods for managing feeds as
/// well as types for parsing websocket messages.
pub mod websocket;

pub use http::Client as HttpClient;
pub use websocket::Client as WebsocketClient;
