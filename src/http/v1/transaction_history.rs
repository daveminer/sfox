use futures_util::Future;
use serde::Deserialize;

use super::super::{HttpError, HttpVerb, SFox};

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
    ) -> impl Future<Output = Result<Vec<TransactionHistory>, HttpError>> {
        let url = self.url_for_v1_resource("account/transactions");
        self.request(HttpVerb::Get, &url, None)
    }
}
