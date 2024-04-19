# Web Client for SFox.com API

[![CI](https://github.com/daveminer/sfox/actions/workflows/test.yml/badge.svg)](https://github.com/daveminer/sfox/actions/workflows/test.yml)


## Description
`sfox` provides typed, asynchronous wrappers around the [SFox.com API](https://docs.sfox.com/) HTTP calls
as well as [Serde](https://serde.rs/) types for websocket message deserialization.

_FIX is not implemented._

## Installation

Complete the steps in this section to make the `sfox` client available in your Rust application.

#### Environment

Set `SFOX_AUTH_TOKEN` (created in the SFox web console) in your environment:
```
SFOX_AUTH_TOKEN=<AUTH-TOKEN>
```

_Note: The server URLs `SFOX_HTTP_SERVER_URL` and `SFOX_WS_SERVER_URL` are also overridable for testing and development._

#### Dependency

Add the following line under ```[dependencies]``` in your project's `Cargo.toml`:
```
sfox = { git = "https://github.com/daveminer/sfox.git", version = "0.1.0" }
```

## Usage

The ```sfox::http``` module performs asynchronous calls to the SFox API and returns typed responses.

#### HTTP

```
use sfox::http::{self, v1::order_book::OrderBook};
```

```
let sfox = http::new().unwrap();
let order_book: OrderBook = sfox.order_book("btcusd").await.unwrap();
println!("Order book currency: {:?}", order_book.bids[0]);
```

The terminal should then print a response like:
```
Order book currency: OpenOrder { price: 35000.012, volume: 1.0, exchange: "some-exchange" }
```

#### Websocket


Usage of the WebSocket client includes instantiating the client, authenticating with the server,
and subscribing/unsubscribing to feeds.

```
use sfox::websocket::Client;
```
```
let sfox_ws = Client::new().await?;
let (mut write, mut read) = sfox_ws.stream.split();

// Start a task to read messages from the SFox stream
let _sfox_handle = tokio::spawn(async move { handle_incoming_message(&mut read).await });

// Subscribe to a feed on the websocket server
let _ticker_subscription = Client::subscribe(&mut write, Feed::Ticker, vec!["btcusd".to_string()]).await;

// Authenticate to access private feeds
let _authentication_attempt = Client::authenticate(&mut write).await;

// Subscribe to a private feed
let _balance_subscription = Client::subscribe(&mut write, Feed::Balances, vec![]).await;
```

where `handle_incoming_message` could be implemented like:
```
async fn handle_incoming_message(read: &mut SplitStream<WssStream> ) {
    while let Some(message) = read.next().await {
        println!("Received message: {:?}", message);
    }
}
```

## Minimum Supported Rust Version (MSRV)

The current MSRV is 1.69. This version may change in future minor versions, so use a restricted version requirement if a specific Rust version is required.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thank you!

1. Fork the Project
2. Create your Feature Branch
```git checkout -b feature/AmazingFeature```
3. Commit your Changes
```git commit -m 'Add some AmazingFeature'```
4. Push to the Branch
```git push origin feature/AmazingFeature```
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.
