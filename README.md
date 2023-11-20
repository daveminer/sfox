# Web Client for SFox.com API

[![CI](https://github.com/daveminer/sfox/actions/workflows/test.yml/badge.svg)](https://github.com/daveminer/sfox/actions/workflows/test.yml)


## Description  
`sfox` provides typed, asynchronous wrappers around the [SFox.com API](https://docs.sfox.com/) HTTP calls
as well as Serde types for websocket message deserialization.  

HTTP - :heavy_check_mark:  
Websocket  - :heavy_check_mark:  
FIX - :x:

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

...

let sfox = http::new().unwrap();
let order_book: OrderBook = sfox.order_book("btcusd").await.unwrap();
println!("Order book currency: {:?}", order_book.bids[0]);
```

The terminal should then print a response like:
```
Order book currency: OpenOrder { price: 35000.012, volume: 1.0, exchange: "some-exchange" }
```

#### Websocket

_In Development_
