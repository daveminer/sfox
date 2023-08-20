use std::collections::HashMap;

use futures_util::Future;
use serde::Deserialize;

use super::Client;
use crate::http::{HttpError, HttpVerb};

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
        let url = self.url_for_v1_resource("post-trade-settlement");

        self.request(HttpVerb::Get, &url, None)
    }

    pub fn post_trade_settlement_interest(
        self,
    ) -> impl Future<Output = Result<HashMap<String, PostTradeSettlementInterest>, HttpError>> {
        let url = self.url_for_v1_resource("post-trade-settlement/interest");
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn post_trade_settlement_positions(
        self,
        status: Option<String>,
    ) -> impl Future<Output = Result<PostTradeSettlementPositions, HttpError>> {
        let resource = "post-trade-settlement/positions";

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
