use futures_util::Future;
use std::collections::HashMap;

use super::super::{Client, HttpError, HttpVerb};

impl Client {
    pub fn ach_bank_transfer(self, amount: f64) -> impl Future<Output = Result<(), HttpError>> {
        let mut params = HashMap::new();
        params.insert("amount".into(), amount.to_string());

        let url = self.url_for_v1_resource("user/bank/deposit");
        self.request(HttpVerb::Post, &url, Some(&params))
    }
}
