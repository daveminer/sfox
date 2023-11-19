# Web Client for SFox.com API

[![CI](https://github.com/daveminer/sfox/actions/workflows/test.yml/badge.svg)](https://github.com/daveminer/sfox/actions/workflows/test.yml)


HTTP - :heavy_check_mark:
Websocket  - :heavy_check_mark:
FIX - :x:

## Description

`sfox` provides typed, asynchronous wrappers around the [SFox.com API](https://www.sfox.com/api/docs) HTTP calls
as well as Serde types for websocket message deserialization.

## Installation

Import into your Rust project:

Set `SFOX_AUTH_TOKEN` in your environment:

```
SFOX_AUTH_TOKEN=<AUTH-TOKEN>
```

The server URLs are overridable for non-production use:
`SFOX_HTTP_SERVER_URL`
`SFOX_WS_SERVER_URL`

## Usage

