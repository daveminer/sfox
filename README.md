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

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thank you!

Fork the Project
Create your Feature Branch (git checkout -b feature/AmazingFeature)
Commit your Changes (git commit -m 'Add some AmazingFeature')
Push to the Branch (git push origin feature/AmazingFeature)
Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.
