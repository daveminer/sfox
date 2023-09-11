use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

static WITHDRAW_RESOURCE: &str = "user/withdraw";

#[derive(Clone, Debug, Deserialize)]
pub struct Withdrawal {
    pub address: String,
    pub atx_id: usize,
    pub amount: f64,
    pub currency: String,
    pub success: bool,
    pub tx_status: usize,
}

impl Client {
    pub fn withdraw(
        self,
        address: &str,
        amount: f64,
        currency: &str,
        is_wire: bool,
    ) -> impl Future<Output = Result<Withdrawal, HttpError>> {
        let mut params = HashMap::new();
        params.insert("address".to_string(), address.to_string());
        params.insert("amount".to_string(), amount.to_string());
        params.insert("currency".to_string(), currency.to_string());
        params.insert("isWire".to_string(), is_wire.to_string());

        let url = self.url_for_v1_resource(WITHDRAW_RESOURCE);

        self.request(HttpVerb::Post, &url, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const WITHDRAW_RESPONSE_BODY: &str = r#"
        {
            "success": true,
            "id": "5pauoj52osolwphwnioqxx2zcekikm23x2hyq2rkotockiysng3y5k245q",
            "atx_id": 1524562,
            "tx_status": 1100,
            "currency": "eth",
            "amount": 0.1,
            "address": "0x12345"
        }
    "#;

    #[tokio::test]
    async fn withdraw_test() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: WITHDRAW_RESPONSE_BODY.into(),
            path: format!("/v1/{}", WITHDRAW_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.withdraw("0x00", 123.45, "btc", true).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
