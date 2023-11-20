use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub balance: f64,
    pub available: f64,
    pub held: f64,
    pub borrow_wallet: f64,
    pub collateral_wallet: f64,
    pub lending_wallet: f64,
    pub trading_wallet: f64,
}

static RESOURCE: &str = "user/balance";

impl Client {
    pub fn account_balance(self) -> impl Future<Output = Result<Vec<AccountBalance>, HttpError>> {
        let url = self.url_for_v1_resource(RESOURCE);
        self.request::<Vec<AccountBalance>>(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_test_server_and_client, ApiMock};

    const RESPONSE_BODY: &str = r#"
               [
                 {
                   "currency": "USD",
                   "balance": 140.55,
                   "available": 130.55,
                   "held": 10.0,
                   "borrow_wallet": 20.0,
                   "collateral_wallet": 30.0,
                   "lending_wallet": 40.0,
                   "trading_wallet": 50.0
                 },
                 {
                   "currency": "USD",
                   "balance": 0,
                   "available": 0,
                   "held": 0,
                   "borrow_wallet": 0,
                   "collateral_wallet": 0,
                   "lending_wallet": 0,
                   "trading_wallet": 0
                 }
               ]
           "#;

    #[tokio::test]
    async fn test_account_balance() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: RESPONSE_BODY.into(),
            path: format!("/v1/{}", RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;
        let result = client.account_balance().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
