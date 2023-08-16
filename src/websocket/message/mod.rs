use serde::de::Error as DeserializeError;
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::{Error, Value};

use self::{
    account::WsBalanceResponsePayload, auth::WsAuthResponse, market::WsSubscriptionResponse,
};

pub mod account;
pub mod auth;
pub mod market;

#[derive(Deserialize)]
pub struct WsResponse<T> {
    pub recipient: String,
    pub sequence: usize,
    pub timestamp: usize,
    pub payload: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeMsg {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub feeds: Vec<String>,
}

#[derive(Deserialize)]
pub enum ResponseMsg<T> {
    Auth(WsAuthResponse),
    Subscribe(WsSubscriptionResponse),
    Unsubscribe(WsSubscriptionResponse),
    Feed(WsResponse<T>),
}

trait Deser {
    fn parse(&self);
}

//impl Deser for ResponseMsg {
// fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
// where
//     D: Deserializer<'de>,
// {
// }
//}

pub enum SubscribeAction {
    Subscribe,
    Unsubscribe,
}

impl Into<String> for SubscribeAction {
    fn into(self) -> String {
        match self {
            SubscribeAction::Subscribe => "subscribe".to_string(),
            SubscribeAction::Unsubscribe => "unsubscribe".to_string(),
        }
    }
}

pub fn parse_message(msg: String) -> Result<Box<dyn Deser>, Error> {
    // Attempt to parse "recipient" out of the message
    let json: serde_json::Value = serde_json::from_str(&msg)?;

    match json["recipient"].as_str() {
        Some(recipient) => parse_feed_msg(json, recipient),
        // Some("account") => Ok(ResponseMsg::WsSubscriptionResponse),
        // Some("orderbook") => Ok(ResponseMsg::WsResponse),
        None => parse_account_msg(json),
    }
}

fn parse_account_msg(msg: Value) -> Result<Box<dyn Deser>, Error> {
    let action = msg["payload"]["action"].as_str();
    let msg_struct = match action {
        //Some("authenticate") => {
        //    let p = serde_json::from_value::<WsAuthResponsePayload>(msg["payload"].clone())?;
        //    Ok(p)
        //parse_feed_msg::<WsAuthResponse>(msg)
        //} //Ok(ResponseMsg::WsAuthResponse),
        Some("subscribe") => {
            let response: WsSubscriptionResponse = serde_json::from_value(msg)?;
            Ok(ResponseMsg::Subscribe(response))
        }
        Some("unsubscribe") => {
            let response: WsSubscriptionResponse = serde_json::from_value(msg)?;
            Ok(ResponseMsg::Unsubscribe(response))
        }
        _ => Err(Error::custom("Unknown response type")),
    };

    Ok(msg_struct)
}

fn parse_feed_msg<T>(msg: Value, recipient: &str) -> Result<Box<dyn Deser>, Error> {
    match recipient {
        "private.user.balances" => {
            let balance_resp: ResponseMsg::WsResponse<WsBalanceResponsePayload> =
                serde_json::from_value(msg)?;
            Ok(balance_resp)
        }
        "orderbook" => Ok(ResponseMsg::WsResponse),
        _ => Err(Error::custom(format!(
            "Unknown recipient type: {}",
            recipient
        ))),
    }
}

// fn parse_feed_msg<T>(msg: Value) -> Result<WsResponse<T>, Error>
// where
//     T: DeserializeOwned,
// {
//     let response: WsResponse<T> = serde_json::from_value(msg)?;
//     Ok(response)
// }
