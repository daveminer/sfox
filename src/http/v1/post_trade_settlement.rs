use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::Client;
use crate::http::{HttpError, HttpVerb};

static POST_TRADE_SETTLEMENT_RESOURCE: &str = "post-trade-settlement";
static POST_TRADE_SETTLEMENT_INTEREST_RESOURCE: &str = "post-trade-settlement/interest";
static POST_TRADE_SETTLEMENT_POSITIONS_RESOURCE: &str = "post-trade-settlement/positions";

#[derive(Clone, Debug, Deserialize)]
pub struct PostTradeSettlement {
    pub exposure: f64,
    pub available_exposure: f64,
    pub exposure_limit: f64,
    pub equity: f64,
    pub equity_for_withdrawals: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostTradeSettlementInterest {
    pub interest_rate: f64,
    pub interest_grace_period_minutes: usize,
    pub interest_frequency_minutes: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostTradeSettlementPositions {
    pub id: usize,
    pub status: String,
    pub date_added: String,
    pub date_loan_closed: Option<String>,
    pub loan_currency_symbol: String,
    pub current_loan_qty: f64,
    pub collateral_currency: String,
    pub pair: String,
    pub interest_rate: f64,
    pub interest_qty: f64,
    pub margin_type: String,
    pub order_id_open: usize,
    pub order_id_close: Option<usize>,
    pub proceeds: f64,
    pub vwap: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WalletTransfer {
    pub from_transaction_id: usize,
    pub to_transaction_id: usize,
    pub currency: String,
    pub quantity: String,
    pub from_wallet: String,
    pub to_wallet: String,
}

impl Client {
    pub fn post_trade_settlement(
        self,
    ) -> impl Future<Output = Result<PostTradeSettlement, HttpError>> {
        let url = self.url_for_v1_resource(POST_TRADE_SETTLEMENT_RESOURCE);

        self.request(HttpVerb::Get, &url, None)
    }

    pub fn post_trade_settlement_interest(
        self,
    ) -> impl Future<Output = Result<HashMap<String, PostTradeSettlementInterest>, HttpError>> {
        let url = self.url_for_v1_resource(POST_TRADE_SETTLEMENT_INTEREST_RESOURCE);
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn post_trade_settlement_positions(
        self,
        status: Option<String>,
    ) -> impl Future<Output = Result<PostTradeSettlementPositions, HttpError>> {
        let resource = POST_TRADE_SETTLEMENT_POSITIONS_RESOURCE;

        let query_str = match status {
            Some(s) => format!("{}?status={}", resource, s),
            None => resource.into(),
        };

        let url = self.url_for_v1_resource(&query_str);

        self.request(HttpVerb::Get, &url, None)
    }

    pub fn wallet_transfer(
        self,
        currency: String,
        quantity: f64,
        from_wallet: String,
        to_wallet: String,
    ) -> impl Future<Output = Result<WalletTransfer, HttpError>> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("quantity", quantity.to_string());
        params.insert("from_wallet", from_wallet);
        params.insert("to_wallet", to_wallet);

        let url = self.url_for_v1_resource("account/transfer");
        self.request(HttpVerb::Post, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_test_server_and_client, ApiMock};

    const POST_TRADE_SETTLEMENT_RESPONSE_BODY: &str = r#"
        {
            "exposure": 1000378.13,
            "available_exposure":1999489.96,
            "exposure_limit":2000000.00,
            "equity": 100221.12,
            "equity_for_withdrawals":8000.42
        }
    "#;

    const POST_TRADE_SETTLEMENT_INTEREST_RESPONSE_BODY: &str = r#"
        {
            "usd": {
                "interest_rate":0.02,
                "interest_frequency_minutes":60,
                "interest_grace_period_minutes":1440
            },
            "btc": {
                "interest_rate":0.03,
                "interest_frequency_minutes":60,
                "interest_grace_period_minutes":1440
            }
        }
    "#;

    const POST_TRADE_SETTLEMENT_POSITIONS_RESPONSE_BODY: &str = r#"
        {
            "id": 3065,
            "status": "ACTIVE",
            "date_added": "2022-06-01T19:49:53.000Z",
            "date_loan_closed": null,
            "loan_currency_symbol": "btc",
            "current_loan_qty": 0.03349122,
            "collateral_currency": "usd",
            "pair": "btcusd",
            "interest_rate": 0.1,
            "interest_qty": 0.0000931,
            "margin_type": "PTS_SHORT",
            "order_id_open": 1117465,
            "order_id_close": null,
            "proceeds": 1003.651384,
            "vwap": 29967.597
        }
    "#;

    #[tokio::test]
    async fn test_post_trade_settlement() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: POST_TRADE_SETTLEMENT_RESPONSE_BODY.into(),
            path: format!("/v1/{}", POST_TRADE_SETTLEMENT_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client.post_trade_settlement().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_post_trade_settlement_interest() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: POST_TRADE_SETTLEMENT_INTEREST_RESPONSE_BODY.into(),
            path: format!("/v1/{}", POST_TRADE_SETTLEMENT_INTEREST_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client.post_trade_settlement_interest().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_post_trade_settlement_positions() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: POST_TRADE_SETTLEMENT_POSITIONS_RESPONSE_BODY.into(),
            path: format!(
                "/v1/{}?status=closed",
                POST_TRADE_SETTLEMENT_POSITIONS_RESOURCE
            ),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .post_trade_settlement_positions(Some("closed".into()))
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
