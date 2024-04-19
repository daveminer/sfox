use std::collections::HashMap;

use futures_util::Future;
use serde_derive::Deserialize;

use crate::http::{Client, HttpError, HttpVerb};

static TRANSACTION_HISTORY_RESOURCE: &str = "account/transactions";
static ORDERS_REPORT_RESOURCE: &str = "orders/buy";
static MONTHLY_SUMMARY_BY_ASSET_RESOURCE: &str = "users/reports/tax-currency-summary";

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionHistory {
    pub id: usize,
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
    #[serde(rename = "AccountTransferFee")]
    pub account_transfer_fee: f64,
    #[serde(rename = "Description")]
    pub description: String,
    pub added_by_user_email: String,
    pub timestamp: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Started,
    ApprovalRequired,
    ProcessingAutomaticWithdrawal,
    Confirmed,
    Done,
    Canceled,
    AdminHoldPendingReview,
}

impl Client {
    pub fn transaction_history(
        self,
        from: Option<String>,
        to: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
        types: Option<String>,
    ) -> impl Future<Output = Result<Vec<TransactionHistory>, HttpError>> {
        let query_str = self.url_for_v1_resource(TRANSACTION_HISTORY_RESOURCE);

        let mut params = HashMap::new();
        if let Some(from) = from {
            params.insert("from".to_string(), from);
        }
        if let Some(to) = to {
            params.insert("to".to_string(), to);
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        if let Some(offset) = offset {
            params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(types) = types {
            params.insert("types".to_string(), types);
        }

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn orders_report(
        self,
        end: usize,
        start: usize,
    ) -> impl Future<Output = Result<String, HttpError>> {
        let query_str = self.url_for_v1_resource(ORDERS_REPORT_RESOURCE);

        let mut params = HashMap::new();
        params.insert("start".to_string(), start);
        params.insert("end".to_string(), end);

        self.request_text(HttpVerb::Get, &query_str, None)
    }

    pub fn monthly_summary_by_asset(
        self,
        currency: String,
        end: Option<usize>,
        start: Option<usize>,
    ) -> impl Future<Output = Result<String, HttpError>> {
        let query_str = self.url_for_v1_resource(MONTHLY_SUMMARY_BY_ASSET_RESOURCE);

        let mut params = HashMap::new();
        params.insert("currency".to_string(), currency);
        if let Some(ts) = end {
            params.insert("end".to_string(), ts.to_string());
        }
        if let Some(ts) = start {
            params.insert("start".to_string(), ts.to_string());
        }

        self.request_text(HttpVerb::Get, &query_str, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_test_server_and_client, ApiMock};

    const TRANSACTION_HISTORY_RESPONSE_BODY: &str = r#"
    [
        {
            "id": 1023696,
            "order_id": "101",
            "client_order_id": "b60d6065-b8be-4a90-8fcb-9962cd1904ba",
            "day": "2021-10-20T17:36:01.000Z",
            "action": "Buy",
            "currency": "usd",
            "memo": "",
            "amount": -438.34854806,
            "net_proceeds": -438.34854806,
            "price": 465.19547184,
            "fees": 1.53,
            "status": "done",
            "hold_expires": "",
            "tx_hash": "",
            "algo_name": "Market",
            "algo_id": "100",
            "account_balance": 0.00029835,
            "AccountTransferFee": 0,
            "Description": "",
            "wallet_display_id": "5a3f1b1c-719d-11e9-b0be-0ea0e44d1000",
            "added_by_user_email": "trader@email.com",
            "timestamp": 1634751361000
        }
    ]
    "#;

    const ORDERS_REPORT_RESPONSE_BODY: &str = r#"OrderId,OrderDate,AddedByUserEmail,Action,AssetPair,Quantity,Asset,AssetUSDFXRate,UnitPrice,PriceCurrency,PrincipalAmount,PriceUSDFXRate,PrincipalAmountUSD,Fees,FeesUSD,Total,TotalUSD
703915618,Tue Jan 24 2023 01:44:00 GMT+0000 (Coordinated Universal Time),qmccarthy@sfox.com,Buy,xlmusd,106.35165019,xlm,0.09384001,0.09384001,usd,9.98003992,1,9.98003992,0.01996008,0.01996008,10,10
703915655,Tue Jan 24 2023 01:47:06 GMT+0000 (Coordinated Universal Time),qmccarthy@sfox.com,Sell,xlmusd,150.56406,xlm,0.09377324,0.09377324,usd,14.11887974,1,14.11887973,0.02823776,0.02823776,14.09064197,14.09064197
704255180,Fri Feb 10 2023 00:19:10 GMT+0000 (Coordinated Universal Time),qmccarthy@sfox.com,Buy,avaxusd,1,avax,17.94148758,17.94148758,usd,17.94148758,1,17.94148758,0.03588298,0.03588298,17.97737056,17.97737056
"#;

    const MONTHLY_SUMMARY_BY_ASSET_RESPONSE_BODY: &str = r#"CurrencyYear,CurrencyMonth,Currency,Deposits,DepositsUSD,Credits,Withdrawals,WithdrawalsUSD,Charges,Buys,BuysTotalUSD,BuysTotalFeesUSD,Sells,SellsTotalUSD,SellsTotalFeesUSD,BuysForCrypto,BuysForCryptoUSD,SellsForCrypto,SellsForCryptoUSD
2023,1,usd,0,0,0,0,0,0,0,0,0,0,0,0,14.11887974,14.11887974,9.98003992,9.98003992
2023,2,usd,25,25,0,0,0,0,0,0,0,0,0,0,0,0,17.94148758,17.94148758
"#;

    #[tokio::test]
    async fn test_transaction_history() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: TRANSACTION_HISTORY_RESPONSE_BODY.into(),
            path: format!("/v1/{}", TRANSACTION_HISTORY_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .transaction_history(None, None, None, None, None)
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_orders_report() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: ORDERS_REPORT_RESPONSE_BODY.into(),
            path: format!("/v1/{}", ORDERS_REPORT_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result: Result<String, HttpError> = client.orders_report(703915618, 704255180).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_monthly_summary_by_asset() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: MONTHLY_SUMMARY_BY_ASSET_RESPONSE_BODY.into(),
            path: format!("/v1/{}", MONTHLY_SUMMARY_BY_ASSET_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client
            .monthly_summary_by_asset("btc".into(), Some(703915618), Some(704255180))
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
