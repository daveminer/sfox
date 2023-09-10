use futures_util::Future;
use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::http::{Client, HttpError, HttpVerb};

static APPROVAL_RULES_RESOURCE: &str = "approval-rules";
static APPROVAL_RESOURCE: &str = "approvals";
static CUSTODY_RESOURCE: &str = "whitelisted-addresses";

#[derive(Clone, Debug, Deserialize)]
pub enum ApprovalRuleType {
    #[serde(rename = "ADD_ALTER_COLL")]
    AddAlterColl,

    #[serde(rename = "ALTER_SAFE")]
    AlterSafe,

    #[serde(rename = "WITHDRAW")]
    Withdraw,
}

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
pub struct CustodyAddressesResponse {
    pub data: Vec<CustodyAddress>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CustodyAddress {
    // Only present on POST response
    pub id: Option<String>,
    pub alias: String,
    pub address: String,
    pub currency_symbol: String,
    pub date_created: String,
    pub date_updated: String,
    // Only present on POST response
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalRequestResponse {
    pub data: Vec<ApprovalRequest>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalRequest {
    pub approval_id: usize,
    pub requested_by_username: String,
    pub requested_by_uaid: String,
    pub date_added: String,
    pub status: String,
    pub approval_type: ApprovalRuleType,
    pub required_approvals: usize,
    pub received_approvals: usize,
    pub action_details: ActionDetails,
    pub approval_responses: ApprovalResponse,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionDetails {
    pub atx_currency_code: String,
    pub atx_amount: f64,
    pub atx_dest_address: String,
    pub threshold: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalResponse {
    pub ua_display_id: String,
    pub username: String,
    pub approved: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalRulesResponse {
    pub data: Vec<ApprovalRule>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalRule {
    pub id: usize,
    pub rule_type: ApprovalRuleType,
    pub date_added: String,
    pub status: String,
    pub available_approver_count: usize,
    pub required_approvals: usize,
    pub threshold: usize,
}

impl Client {
    pub fn custody_addresses(
        self,
    ) -> impl Future<Output = Result<CustodyAddressesResponse, HttpError>> {
        let query_str = self.url_for_v1_resource(CUSTODY_RESOURCE);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn add_custody_address(
        self,
        alias: String,
        currency_symbol: String,
        address: String,
    ) -> impl Future<Output = Result<CustodyAddress, HttpError>> {
        let query_str = self.url_for_v1_resource(CUSTODY_RESOURCE);

        let mut params = HashMap::new();
        params.insert("alias".into(), alias);
        params.insert("currency_symbol".into(), currency_symbol);
        params.insert("address".into(), address);

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }

    pub fn approval_rules(self) -> impl Future<Output = Result<ApprovalRulesResponse, HttpError>> {
        let query_str = self.url_for_v1_resource(APPROVAL_RULES_RESOURCE);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn add_approval_rule(
        self,
        rule_type: String,
        required_approvals: usize,
        threshold: usize,
    ) -> impl Future<Output = Result<ApprovalRule, HttpError>> {
        let query_str = self.url_for_v1_resource(APPROVAL_RULES_RESOURCE);

        let mut params = HashMap::new();
        params.insert("rule_type".to_string(), rule_type);
        params.insert(
            "required_approvals".to_string(),
            required_approvals.to_string(),
        );
        params.insert("threshold".to_string(), threshold.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }

    pub fn edit_approval_rule(
        self,
        id: usize,
        required_approvals: usize,
        threshold: f64,
    ) -> impl Future<Output = Result<ApprovalRule, HttpError>> {
        let query_str = self.url_for_v1_resource(&format!("{}/{}", APPROVAL_RULES_RESOURCE, id));

        let mut params = HashMap::new();
        params.insert("required_approvals".into(), required_approvals.to_string());
        params.insert("threshold".into(), threshold.to_string());

        self.request(HttpVerb::Patch, &query_str, Some(&params))
    }

    pub fn approval_requests(
        self,
        pending: bool,
    ) -> impl Future<Output = Result<ApprovalRequestResponse, HttpError>> {
        let mut query_str = self.url_for_v1_resource(APPROVAL_RESOURCE);
        if pending {
            query_str = format!("{}?pending=true", query_str);
        }

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn respond_to_approval_request(
        self,
        id: usize,
        approve: bool,
    ) -> impl Future<Output = Result<(), HttpError>> {
        let query_str = self.url_for_v1_resource(&format!("{}/{}", APPROVAL_RESOURCE, id));

        // TODO: handle polymorphic params
        let mut params = HashMap::new();
        params.insert("approve".to_string(), approve.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::server::{new_server_and_client, ApiMock};

    const CUSTODY_ADDRESSES_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "alias": "Satoshis Fund",
                    "address": "1NLqQmwkGxxQmzS9uwtCGXxbxrcNW4FpYp",
                    "currency_symbol": "btc",
                    "date_created": "2021-09-15T15:12:13.000Z",
                    "date_updated": "2021-09-15T15:12:13.000Z",
                    "status": "Pending"
                }
            ]
        }
    "#;

    const CREATE_CUSTODY_ADDRESS_RESPONSE_BODY: &str = r#"
        {
            "id": "4d90ee41-3b36-11ec-bdb0-0ab29ff926a1",
            "alias": "Satoshis Fund",
            "address": "1NLqQmwkGxxQmzS9uwtCGXxbxrcNW4FpYp",
            "currency_symbol": "btc",
            "date_created": "2021-11-01T17:08:15.000Z",
            "date_updated": "2021-11-01T17:08:15.000Z",
            "status": "Pending",
            "tag": null
        }
    "#;

    const APPROVAL_RULES_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "id": 3,
                    "available_approver_count": 2,
                    "date_added": "2021-03-17T16:19:47.000Z",
                    "required_approvals": 2,
                    "rule_type": "WITHDRAW",
                    "status": "Pending Approval",
                    "threshold": 20
                }
            ]
        }
    "#;

    const APPROVAL_RULE_RESPONSE_BODY: &str = r#"
        {
            "id": 1,
            "rule_type": "WITHDRAW",
            "date_added": "2021-03-17T16:19:47.000Z",
            "status": "Pending",
            "available_approver_count": 2,
            "required_approvals": 2,
            "threshold": 0
        }
    "#;

    const APPROVALS_RESPONSE_BODY: &str = r#"
        {
            "data": [
                {
                    "approval_id": 1,
                    "requested_by_username": "example@email.com",
                    "requested_by_uaid": "6ea3fb9e-7797-11eb-aa51-0242ac120002",
                    "date_added": "2021-03-03T20:28:41.000Z",
                    "status": "Pending",
                    "approval_type": "WITHDRAW",
                    "required_approvals": 2,
                    "received_approvals": 0,
                    "action_details": {
                        "atx_currency_code": "btc",
                        "atx_amount": 5,
                        "atx_dest_address": "0x1232131223",
                        "threshold": 1
                    },
                    "approval_responses": {
                        "ua_display_id": "3fb9e6ea-7797-11eb-aa51-200020242ac1",
                        "username": "collaborator@email.com",
                        "approved": true
                    }
                }
            ]
        }
    "#;

    #[tokio::test]
    async fn test_custody_addresses() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: CUSTODY_ADDRESSES_RESPONSE_BODY.into(),
            path: format!("/v1/{}", CUSTODY_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.custody_addresses().await;
        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_create_custody_addresses() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: CREATE_CUSTODY_ADDRESS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", CUSTODY_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .add_custody_address("test alias".into(), "btc".into(), "0x123".into())
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_approval_rules() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: APPROVAL_RULES_RESPONSE_BODY.into(),
            path: format!("/v1/{}", APPROVAL_RULES_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.approval_rules().await;
        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_new_approval_rule() {
        let mock = ApiMock {
            action: HttpVerb::Post,
            body: APPROVAL_RULE_RESPONSE_BODY.into(),
            path: format!("/v1/{}", APPROVAL_RULES_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client
            .add_approval_rule("test rule type".into(), 1, 1)
            .await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_edit_approval_rule() {
        let rule_id = 1;

        let mock = ApiMock {
            action: HttpVerb::Patch,
            body: APPROVAL_RULE_RESPONSE_BODY.into(),
            path: format!("/v1/{}/{}", APPROVAL_RULES_RESOURCE, rule_id),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.edit_approval_rule(rule_id, 2, 2.0).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_approval_requests() {
        let mock = ApiMock {
            action: HttpVerb::Get,
            body: APPROVALS_RESPONSE_BODY.into(),
            path: format!("/v1/{}", APPROVAL_RESOURCE),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.approval_requests(false).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn test_approval_request_response() {
        let request_id = 1;

        let mock = ApiMock {
            action: HttpVerb::Post,
            body: r#"null"#.into(),
            path: format!("/v1/{}/{}", APPROVAL_RESOURCE, request_id),
            response_code: 200,
        };

        let (client, _server, mock_results) = new_server_and_client(vec![mock]).await;

        let result = client.respond_to_approval_request(request_id, true).await;

        assert!(result.is_ok());

        for mock in mock_results {
            mock.assert_async().await;
        }
    }
}
