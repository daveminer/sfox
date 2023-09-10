use std::pin::Pin;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{Client, HttpError, HttpVerb};

static METRICS_RESOURCE: &str = "margin/account";
static POSITIONS_RESOURCE: &str = "margin/loans";

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
        let query_str = self.url_for_v1_resource(METRICS_RESOURCE);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn loan_positions(
        self,
        status: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<LoanPositionResponse, HttpError>>>> {
        let resource = POSITIONS_RESOURCE;
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const POSITIONS_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "id": 3835,
                    "status": "CLOSED",
                    "date_added": "2022-06-30T01:01:55.000Z",
                    "date_loan_closed": "2022-06-30T01:02:47.000Z",
                    "loan_currency": "btc",
                    "collateral_currency": "usd",
                    "pair": "btcusd",
                    "original_collateral_qty": 2101.94565,
                    "current_collateral_qty": 0,
                    "original_loan_qty": 0.1,
                    "current_loan_qty": 0,
                    "interest_qty": 0,
                    "interest_rate": 0.1,
                    "margin_type": "MARGIN_SHORT",
                    "order_id": 1127584,
                    "proceeds": 0
                },
                {
                    "id": 3836,
                    "status": "CLOSED",
                    "date_added": "2022-06-30T02:46:57.000Z",
                    "date_loan_closed": "2022-07-01T02:41:01.000Z",
                    "loan_currency": "btc",
                    "collateral_currency": "usd",
                    "pair": "btcusd",
                    "original_collateral_qty": 2107.60095,
                    "current_collateral_qty": 0,
                    "original_loan_qty": 0.1,
                    "current_loan_qty": 0,
                    "interest_qty": 0,
                    "interest_rate": 0.1,
                    "margin_type": "MARGIN_SHORT",
                    "order_id": 1127591,
                    "proceeds": 0
                }
            ]
        }
    "#;

    const METRICS_RESPONSE_BODY: &str = r#"
        {
            "account_value": 100885.75696235,
            "equity": 97850.05595684,
            "position_notional": 88153.20356472,
            "collateral": 97850.05595684,
            "free_collateral": 97850.05595684,
            "margin_level": 1.11,
            "margin_call_level": 0.15,
            "maintenance_margin_level": 0.05
        }
    "#;

    #[tokio::test]
    async fn test_loan_positions() {
        let filter = Some("active".to_string());
        let mut path = format!("/v1/{}", POSITIONS_RESOURCE);

        if filter.clone().is_some() {
            path = format!("{}?status={}", path, filter.clone().unwrap());
        }

        let mock = ApiMock {
            action: HttpVerb::Get,
            body: POSITIONS_RESPONSE_BODY.into(),
            path,
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.loan_positions(filter).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_loan_metrics() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: METRICS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", METRICS_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.loan_metrics().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
