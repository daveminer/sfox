use std::collections::HashMap;

use futures_util::Future;

use super::super::{Client, HttpError, HttpVerb};

impl Client {
    pub fn ach_bank_transfer(self, amount: f64) -> impl Future<Output = Result<(), HttpError>> {
        let mut params = HashMap::new();
        params.insert("amount".into(), amount.to_string());

        self.request(HttpVerb::Post, "user/bank/deposit", Some(&params))
    }
}
