use futures_util::Future;
use serde::Deserialize;

use super::super::{Client, HttpError, HttpVerb};

static FEE_RESOURCE: &str = "account/fee-rates";
static WITHDRAW_FEE_RESOURCE: &str = "withdraw-fee";

#[derive(Clone, Debug, Deserialize)]
pub struct Fees {
    pub volume: f64,
    #[serde(rename = "makerRate")]
    pub maker_rate: f64,
    #[serde(rename = "nprRate")]
    pub npr_rate: f64,
    #[serde(rename = "nprOffRate")]
    pub npr_off_rate: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WithdrawFee {
    pub fee: f64,
}

impl Client {
    pub fn fees(self) -> impl Future<Output = Result<Fees, HttpError>> {
        let url = self.url_for_v1_resource(FEE_RESOURCE);
        self.request(HttpVerb::Get, &url, None)
    }

    pub fn withdraw_fee(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<WithdrawFee, HttpError>> {
        let url = self.url_for_v1_resource(&format!("{}/{}", WITHDRAW_FEE_RESOURCE, currency));
        self.request(HttpVerb::Get, &url, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_test_server_and_client, ApiMock};

    const FEES_RESPONSE_BODY: &str = r#"
        {
            "volume": 2302868.809741,
            "makerRate": 0.00021,
            "nprRate": 0.00035,
            "nprOffRate": 0.00105
        }
    "#;

    const FEES_INVALID_TOKEN_RESPONSE_BODY: &str = r#"
        {
            "error": "invalid token. check authorization header."
        }
    "#;

    const WITHDRAW_FEE_RESPONSE_BODY: &str = r#"
        {
            "fee": 0.001
        }
    "#;

    #[tokio::test]
    async fn test_fees_ok() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: FEES_RESPONSE_BODY.into(),
            path: format!("/v1/{}", FEE_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client.fees().await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_fees_invalid_token() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: FEES_INVALID_TOKEN_RESPONSE_BODY.into(),
            path: format!("/v1/{}", FEE_RESOURCE),
            response_code: 401,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client.fees().await;

        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string()
                == "Error while making request: `\"invalid token. check authorization header.\"`"
        );

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_withdraw_fee() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: WITHDRAW_FEE_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", WITHDRAW_FEE_RESOURCE, "eth"),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_test_server_and_client(vec![mock]).await;

        let result = client.withdraw_fee("eth").await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
