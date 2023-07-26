use futures_util::Future;
use serde::Deserialize;

//use crate::http::ApiResponse;

use super::super::{Client, HttpError};

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

impl Client {
    pub fn account_balance(self) -> impl Future<Output = Result<Vec<AccountBalance>, HttpError>> {
        self.get_request::<Vec<AccountBalance>>("user/balance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_balance() {
        let mut s = mockito::Server::new_async().await;

        let mock = s
            .mock("GET", "/v1/user/balance")
            .with_status(200)
            .with_body(
                r#"
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
           "#,
            )
            .create_async()
            .await;

        // TODO: allow server_url as input
        let mut client = Client::new().unwrap();
        client.server_url = format!("http://{}", s.host_with_port());
        //println!("HWP: {}", s.host_with_port());

        let response = client.account_balance().await;
        println!("RESP: {:?}", response.unwrap());
        //assert!(response.is_ok());
        //let account_balance = response.unwrap();
        //assert_eq!(account_balance[0].currency, "USD");
        //assert_eq!(account_balance[0].balance, 140.0);

        mock.assert();
    }
}
