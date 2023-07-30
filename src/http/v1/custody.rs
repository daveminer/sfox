use futures_util::Future;
use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::http::{HttpError, HttpVerb, SFox};

#[derive(Clone, Debug, Deserialize)]
enum ApprovalRuleType {
    #[serde(rename = "ADD_ALTER_COLL")]
    AddAlterColl,

    #[serde(rename = "ALTER_SAFE")]
    AlterSafe,

    #[serde(rename = "WITHDRAW")]
    Withdraw
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
pub struct CustoryAddressesResponse {
    pub data: Vec<CustodyAddress>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CustodyAddress {
    // Only present on POST response
    pub id: Option<String>,
    pub alias: usize,
    pub address: String,
    pub currency_symbol: String,
    pub date_created: String,
    pub date_updated: String,
    // Only present on POST response
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApprovalRequestResponse {
    pub data: Vec<ApprovalRule>,
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

static APPROVAL_RULES_RESOURCE: &str = "approval-rules";
static APPROVAL_RESOURCE: &str = "approvals";
static CUSTODY_RESOURCE: &str = "whitelisted-addresses";

impl SFox {
    pub fn custody_addresses(self) -> impl Future<Output = Result<LoanMetrics, HttpError>> {
        let query_str = self.url_for_v1_resource(CUSTODY_RESOURCE);

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn add_custody_address(
        self,
        alias: String,
        currency_symbol: String,
        address: String,
    ) -> impl Future<Output = Result<ApprovalRulesResponse, HttpError>> {
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

    pub fn add_approval_rule(self, rule_type: String, required_approvals: usize, threshold: usize) -> impl Future<Output = Result<ApprovalRule, HttpError>> {
        let query_str = self.url_for_v1_resource(APPROVAL_RULES_RESOURCE);

        let mut params = HashMap::new();
        params.insert("rule_type".to_string(), rule_type);
        params.insert("required_approvals".to_string(), required_approvals.to_string());
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
        pending: Option<bool>,
    ) -> impl Future<Output = Result<Vec<ApprovalRequest>, HttpError>> {
        let query_str = match pending {
            Some(true) => format!("{}?pending=true", APPROVAL_RESOURCE),
            _ => APPROVAL_RESOURCE.to_string()
        }

        self.request(HttpVerb::Get, &query_str, None)
    }

    pub fn respond_to_approval_request(
        self,
        id: usize,
        approve: bool
    ) -> impl Future<Output = Result<(), HttpError>> {
        let query_str = self.url_for_v1_resource(&format!("{}/{}", APPROVAL_RESOURCE, id));


        // TODO: handle polymorphic params
        let mut params = HashMap::new();
        params.insert("approve".to_string(), approve.to_string());

        self.request(HttpVerb::Post, &query_str, Some(&params))
    }
}
