use std::pin::Pin;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{Client, HttpError, HttpVerb};

#[derive(Clone, Debug, Deserialize)]
pub struct LoanMetrics {
    pub account_value: f64,
    pub equity: f64,
    pub position_notional: f64,
    pub collateral: f64,
    pub free_collateral: f64,
    pub margin_level: f64,
    pub margin_call_level: f64,
    pub maintenance_margin_level: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoanPositionResponse {
    pub data: Vec<LoanPosition>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoanPosition {
    pub id: usize,
    pub status: String,
    pub date_added: String,
    pub date_loan_closed: String,
    pub loan_currency: String,
    pub collateral_currency: String,
    pub pair: String,
    pub original_collateral_qty: f64,
    pub current_collateral_qty: f64,
    pub original_loan_qty: f64,
    pub current_loan_qty: f64,
    pub interest_qty: f64,
    pub interest_rate: f64,
    pub margin_type: String,
    pub order_id: usize,
    pub proceeds: usize,
}

impl Client {
    pub fn loan_metrics(self) -> impl Future<Output = Result<LoanMetrics, HttpError>> {
        let query_str = self.url_for_v1_resource("margin/account");

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn loan_positions(
        self,
        status: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<LoanPositionResponse, HttpError>>>> {
        let resource = "margin/loans";
        let query = match status {
            Some(s) => {
                if s != "active" && s != "closed" {
                    return Box::pin(async move {
                        Err(HttpError::InvalidRequest(format!("Invalid status: {}", s)))
                    });
                }
                format!("{}?status={}", resource, s)
            }
            None => resource.into(),
        };
        let url = self.url_for_v1_resource(&query);

        Box::pin(self.request(HttpVerb::Get, &url, None))
    }
}
