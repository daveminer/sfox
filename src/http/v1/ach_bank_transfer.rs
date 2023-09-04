use futures_util::Future;
use serde::Deserialize;
use std::collections::HashMap;

use super::super::{Client, HttpError, HttpVerb};

const ACH_BANK_TRANSFER_RESOURCE: &str = "user/bank/deposit";

#[derive(Clone, Debug, Deserialize)]
pub struct AchBankTransfer {
    pub tx_status: usize,
    pub success: bool,
}

impl Client {
    pub fn ach_bank_transfer(
        self,
        amount: f64,
    ) -> impl Future<Output = Result<AchBankTransfer, HttpError>> {
        let mut params = HashMap::new();
        params.insert("amount".into(), amount.to_string());

        let url = self.url_for_v1_resource(ACH_BANK_TRANSFER_RESOURCE);
        self.request::<AchBankTransfer>(HttpVerb::Post, &url, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const RESPONSE_BODY: &str = r#"
        {
            "tx_status": 0,
            "success": true
        }
    "#;

    #[tokio::test]
    async fn test_ach_bank_transfer() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: RESPONSE_BODY.into(),
            path: format!("/v1/{}", ACH_BANK_TRANSFER_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.ach_bank_transfer(100.0).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
