use hyper::{
    client::{Client as HyperClient, HttpConnector, ResponseFuture},
    Body, Method, Request,
};
use serde_derive::Deserialize;

use crate::settings::Settings;

#[derive(Debug, Deserialize)]
pub struct Client {
    #[serde(skip)]
    pub auth_token: String,
    #[serde(skip)]
    pub http_client: HyperClient<HttpConnector>,
    pub server_url: String,
}

impl Client {
    pub fn new() -> Client {
        let http_client = HyperClient::new();
        let settings = Settings::new().unwrap();

        let auth_token = settings.http.api_key;
        let server_url = settings.http.server_url;

        Client {
            auth_token,
            http_client,
            server_url,
        }
    }

    pub fn account_balance(self) -> ResponseFuture {
        self.get_request("user/balance")
    }

    pub fn transaction_history(self) -> ResponseFuture {
        self.get_request("account/transactions")
    }

    pub fn crypto_deposit_address(self, currency: &str) -> ResponseFuture {
        self.get_request(&format!("user/deposit/address/{}", currency))
    }

    pub fn get_currencies(self) -> ResponseFuture {
        self.get_request("currency")
    }

    pub fn get_fees(self) -> ResponseFuture {
        self.get_request("markets/currency-pairs")
    }

    pub fn withdrawal_fee(self, currency: &str) -> ResponseFuture {
        self.get_request(&format!("withdraw-fee/{}", currency))
    }

    fn get_request(self, resource: &str) -> ResponseFuture {
        let url = format!("{}/v1/{}", self.server_url, resource)
            .parse::<hyper::Uri>()
            .unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .body(Body::empty())
            .unwrap();

        self.http_client.request(req)
    }
}
