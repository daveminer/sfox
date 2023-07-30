use std::collections::HashMap;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{HttpError, HttpVerb, SFox};

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionHistory {
    pub id: usize,
    pub atx_id: usize,
    pub order_id: String,
    pub client_order_id: String,
    pub day: String,
    pub action: String,
    pub currency: String,
    pub memo: String,
    pub amount: f64,
    pub net_proceeds: f64,
    pub price: f64,
    pub fees: f64,
    pub status: TransactionStatus,
    pub hold_expires: String,
    pub tx_hash: String,
    pub algo_name: String,
    pub algo_id: String,
    pub account_balance: f64,
    pub account_transfer_fee: f64,
    pub description: String,
    pub added_by_user_email: String,
    pub symbol: String,
    pub timestamp: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TransactionStatus {
    Started,
    ApprovalRequired,
    ProcessingAutomaticWithdrawal,
    Confirmed,
    Done,
    Canceled,
    AdminHoldPendingReview,
}

impl SFox {
    pub fn transaction_history(
        self,
        from: String,
        to: String,
        limit: usize,
        offset: usize,
        types: String,
    ) -> impl Future<Output = Result<Vec<TransactionHistory>, HttpError>> {
        let query_str = self.url_for_v1_resource("account/transactions");

        let mut params = HashMap::new();
        params.insert("from".to_string(), from);
        params.insert("to".to_string(), to);
        params.insert("limit".to_string(), limit.to_string());
        params.insert("offset".to_string(), offset.to_string());
        params.insert("types".to_string(), types);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn orders_report(
        self,
        end: usize,
        start: usize,
    ) -> impl Future<Output = Result<String, HttpError>> {
        let query_str = self.url_for_v1_resource("users/reports/tax-order-summary");

        let mut params = HashMap::new();
        params.insert("start".to_string(), start);
        params.insert("end".to_string(), end);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn monthly_summary_by_asset(
        self,
        currency: String,
        end: Option<usize>,
        start: Option<usize>,
    ) -> impl Future<Output = Result<String, HttpError>> {
        let query_str = self.url_for_v1_resource("users/reports/tax-currency-summary");

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        if let Some(ts) = end {
            params.insert("end".to_string(), ts.to_string());
        }
        if let Some(ts) = start {
            params.insert("start".to_string(), ts.to_string());
        }

        self.request(HttpVerb::Get, &query_str, Some(&params))
    }
}
