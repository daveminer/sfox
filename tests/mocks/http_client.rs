use std::collections::HashMap;

use reqwest::StatusCode;
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::http::Response;

pub fn ok(response: HashMap<String, String>) -> Response<String> {
    let mock_resp: Value = json!(response);
    Response::builder()
        .status(StatusCode::OK)
        .body(mock_resp.to_string())
        .unwrap()
}
