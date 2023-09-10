use futures_util::Future;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;

use crate::http::{Client, HttpError, HttpVerb};

use super::order::ExecutedQuote;

static REQUEST_FOR_QUOTE_RESOURCE: &str = "quote";
static EXECUTE_QUOTE_RESOURCE: &str = "orders/buy";

#[derive(Clone, Debug, Deserialize)]
pub struct Quote {
    pub quote_id: String,
    pub quantity: f64,
    pub amount: f64,
    pub pair: String,
    pub side: String,
    pub date_expiry: String,
    pub date_quote: String,
    pub buy_price: Option<f64>,
    pub sell_price: Option<f64>,
}

impl Client {
    pub fn request_for_quote(
        self,
        pair: &str,
        side: &str,
        quantity: Option<f64>,
        amount: Option<f64>,
        client_quote_id: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Quote, HttpError>>>> {
        let mut params = HashMap::new();
        params.insert("pair".into(), pair.to_string());
        params.insert("side".into(), side.to_string());

        if quantity.is_none() && amount.is_none() {
            return Box::pin(async {
                Err(HttpError::InvalidRequest(
                    "Either quantity or amount must be provided".into(),
                ))
            });
        }

        if let Some(quantity) = quantity {
            params.insert("quantity".into(), quantity.to_string());
        }

        if let Some(amount) = amount {
            params.insert("amount".into(), amount.to_string());
        }

        if let Some(client_quote_id) = client_quote_id {
            params.insert("client_quote_id".into(), client_quote_id.to_string());
        }

        let url = self.url_for_v1_resource(REQUEST_FOR_QUOTE_RESOURCE);

        Box::pin(self.request(HttpVerb::Post, &url, Some(&params)))
    }

    pub fn execute_quote(
        self,
        currency_pair: &str,
        quantity: f64,
        quote_id: &str,
    ) -> impl Future<Output = Result<ExecutedQuote, HttpError>> {
        let mut params = HashMap::new();
        params.insert("currency_pair".into(), currency_pair.to_string());
        params.insert("quantity".into(), quantity.to_string());
        params.insert("quote_id".into(), quote_id.to_string());

        let url = self.url_for_v1_resource(EXECUTE_QUOTE_RESOURCE);

        self.request(HttpVerb::Post, &url, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const REQUEST_FOR_QUOTE_RESPONSE_BODY: &str = r#"
        {
            "quote_id": "165c404c-ffe9-11ed-b8ed-0a170e3de1bd",
            "pair": "btcusd",
            "side": "BUY",
            "date_expiry": "2023-05-31T19:26:58.595Z",
            "date_quote": "2023-05-31T19:26:48.595Z",
            "amount": 27044.7156,
            "quantity": 1,
            "buy_price": 27044.7156
        }
    "#;

    const EXECUTE_QUOTE_RESPONSE_BODY: &str = r#"
        {
            "id": 1754344,
            "side_id": 500,
            "action": "Buy",
            "algorithm_id": 150,
            "algorithm": "Instant",
            "type": "Instant",
            "pair": "btcusd",
            "quantity": 1,
            "price": 23243.49136824,
            "amount": 0,
            "net_market_amount": 0,
            "filled": 1,
            "vwap": 23243.49136824,
            "filled_amount": 23243.49136824,
            "fees": 0,
            "net_proceeds": -23243.49136824,
            "status": "Done",
            "status_code": 300,
            "routing_option": "BestPrice",
            "routing_type": "None",
            "time_in_force": "FOK",
            "expires": null,
            "dateupdated": "2023-01-26T20:27:06.000Z", "client_order_id": "",
            "user_tx_id": "",
            "o_action": "Buy",
            "algo_id": 150,
            "algorithm_options": null,
            "destination": "",
            "quote_id": "cb436399-9db7-11ed-8ea6-0e5724aafd6b"
        }
    "#;

    #[tokio::test]
    async fn test_request_for_quote() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: REQUEST_FOR_QUOTE_RESPONSE_BODY.into(),
            path: format!("/v1/{}", REQUEST_FOR_QUOTE_RESOURCE),
            response_code: 201,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .request_for_quote("btcusd", "sell", Some(1.0), Some(1.0), Some("123"))
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_execute_quote() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: EXECUTE_QUOTE_RESPONSE_BODY.into(),
            path: format!("/v1/{}", EXECUTE_QUOTE_RESOURCE),
            response_code: 201,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.execute_quote("btcusd", 1.0, "123").await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
