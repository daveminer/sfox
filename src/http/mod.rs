use std::collections::HashMap;
use std::env;

use futures_util::{Future, TryFutureExt};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

/// Candlestick chart data from the SFox markets.
pub mod candlesticks;
/// Core API resources.
pub mod v1;

/// Server URL for candlestick chart data.
pub const DEFAULT_SERVER_URL: &str = "https://api.sfox.com";
/// URL for core API.
pub const CANDLESTICK_SERVER_URL: &str = "https://chartdata.sfox.com";

/// Error type for HTTP requests.
#[derive(Clone, Error, Debug, Deserialize)]
pub enum HttpError {
    #[error("Authentication error: `{0}`")]
    AuthenticationError(String),
    #[error("Could not create http client: `{0}`")]
    InitializationError(String),
    #[error("Invalid request: `{0}`")]
    InvalidRequest(String),
    #[error("Error while making request: `{0}`")]
    TransportError(String),
    #[error("Could not deserialize response. Error: `{0}`, Response: `{1}`")]
    UnparseableResponse(String, String),
}

/// Offers an asynchronous API that models the HTTP resources of the SFox API. Server and authentication are
/// managed internally.
#[derive(Clone, Debug, Deserialize)]
pub struct Client {
    #[serde(skip)]
    pub auth_token: String,
    pub candlestick_server_url: String,
    #[serde(skip)]
    pub http_client: reqwest::Client,
    pub server_url: String,
}

/// GET, POST, PATCH, DELETE
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HttpVerb {
    Get,
    Post,
    Patch,
    Delete,
}

impl From<HttpVerb> for &str {
    fn from(val: HttpVerb) -> Self {
        match val {
            HttpVerb::Get => "GET",
            HttpVerb::Post => "POST",
            HttpVerb::Patch => "PATCH",
            HttpVerb::Delete => "DELETE",
        }
    }
}

/// Builds a new client with the default server URL.
pub fn new() -> Result<Client, HttpError> {
    Client::new()
}

impl Client {
    /// Returns a new client with the default server URL.
    pub fn new() -> Result<Client, HttpError> {
        let candlestick_server_url = env::var("CANDLESTICK_SERVER_URL")
            .unwrap_or_else(|_| CANDLESTICK_SERVER_URL.to_string());

        let server_url =
            env::var("SFOX_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string());

        build_server(candlestick_server_url, server_url)
    }

    /// Builds a new client with the given server URL; useful for testing.
    pub fn new_with_server_url(
        server_url: String,
        candlestick_server_url: String,
    ) -> Result<Client, HttpError> {
        build_server(candlestick_server_url, server_url)
    }

    fn request<T>(
        self,
        verb: HttpVerb,
        resource: &str,
        req_body: Option<&HashMap<String, String>>,
    ) -> impl Future<Output = Result<T, HttpError>>
    where
        T: Clone + DeserializeOwned + Send + 'static,
    {
        self.send_request(verb, resource, req_body)
            .and_then(|response| async move { parse_response(response).await })
    }

    fn request_text(
        self,
        verb: HttpVerb,
        resource: &str,
        req_body: Option<&HashMap<String, String>>,
    ) -> impl Future<Output = Result<String, HttpError>> {
        self.send_request(verb, resource, req_body)
            .and_then(|response| async {
                response
                    .text()
                    .await
                    .map_err(|e| HttpError::TransportError(e.to_string()))
            })
    }

    fn send_request(
        self,
        verb: HttpVerb,
        resource: &str,
        req_body: Option<&HashMap<String, String>>,
    ) -> impl Future<Output = Result<Response, HttpError>> {
        let auth_token = self.auth_token.clone();

        let base_response = self.action(verb.clone(), resource).bearer_auth(auth_token);

        let request = if Self::has_request_body(verb, &req_body) {
            base_response.json(req_body.unwrap())
        } else {
            base_response
        };

        request
            .send()
            .map_err(|e| HttpError::TransportError(e.to_string()))
    }

    fn action(self, verb: HttpVerb, resource_path: &str) -> reqwest::RequestBuilder {
        let c = &self.http_client;
        match verb {
            HttpVerb::Get => c.get(resource_path),
            HttpVerb::Post => c.post(resource_path),
            HttpVerb::Patch => c.patch(resource_path),
            HttpVerb::Delete => c.delete(resource_path),
        }
    }

    fn has_request_body(verb: HttpVerb, req_body: &Option<&HashMap<String, String>>) -> bool {
        match verb {
            HttpVerb::Get | HttpVerb::Delete => false,
            HttpVerb::Post | HttpVerb::Patch => req_body.is_some(),
        }
    }

    fn url_for_candlestick_resource(&self, resource: &str) -> String {
        format!("{}/candlesticks{}", self.candlestick_server_url, resource)
    }

    pub(crate) fn url_for_v1_resource(&self, resource: &str) -> String {
        format!("{}/v1/{}", self.server_url, resource)
    }
}

fn build_server(candlestick_server_url: String, server_url: String) -> Result<Client, HttpError> {
    let http_client = reqwest::Client::builder()
        .build()
        .map_err(|e| HttpError::InitializationError(e.to_string()))?;

    let auth_token = env::var("SFOX_AUTH_TOKEN").map_err(|_| {
        HttpError::InitializationError("SFOX_AUTH_TOKEN env variable not set.".to_string())
    })?;

    Ok(Client {
        auth_token,
        candlestick_server_url,
        http_client,
        server_url,
    })
}

async fn parse_response<T>(response: reqwest::Response) -> Result<T, HttpError>
where
    T: Clone + DeserializeOwned + Send + 'static,
{
    if !response.status().is_success() {
        let error_text = response
            .json()
            .await
            .unwrap_or(json!("{\"error\": \"Could not parse response\"}"));
        let error = error_text.get("error");
        match error {
            Some(error) => {
                return Err(HttpError::TransportError(error.to_string()));
            }
            None => {
                return Err(HttpError::TransportError(error_text.to_string()));
            }
        }
    }

    let text: String = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(HttpError::TransportError(e.to_string())),
    };

    match serde_json::from_str::<T>(&text) {
        Ok(payload) => Ok(payload),
        Err(e) => Err(HttpError::UnparseableResponse(e.to_string(), text)),
    }
}

#[cfg(test)]
mod tests {
    use tokio_tungstenite::tungstenite::http;

    use crate::{
        http::{parse_response, Client, HttpError, DEFAULT_SERVER_URL},
        util::set_test_env,
    };

    #[test]
    fn test_client_initialization() {
        set_test_env();
        let client = Client::new().unwrap();

        // Assert auth token was set correctly
        assert_eq!(client.auth_token, "secret");
        assert_eq!(client.server_url, DEFAULT_SERVER_URL);
    }

    #[test]
    fn test_client_initialization_with_url() {
        set_test_env();
        let server_url = "http://localhost:4000".to_string();
        let candlestick_server_url = "http://localhost:4001".to_string();

        let client =
            Client::new_with_server_url(server_url.clone(), candlestick_server_url).unwrap();

        assert_eq!(client.server_url, server_url);
    }

    #[tokio::test]
    async fn test_parse_response_success() {
        // Create mock response
        let resp = http::Response::builder()
            .status(200)
            // Will not deserialize correctly without quotes
            .body("\"response body\"".to_string())
            .unwrap();

        let parsed: String = parse_response(resp.into()).await.unwrap();
        assert!(!parsed.is_empty())
    }

    #[tokio::test]
    async fn test_parse_response_transport_error() {
        // Create a failed response
        let resp = http::Response::builder()
            .status(400)
            // Will not deserialize correctly without quotes
            .body("\"response body\"")
            .unwrap();

        // Try to parse failed response
        let result = parse_response::<String>(resp.into()).await;

        // Assert error returned
        assert!(matches!(result, Err(HttpError::TransportError(_))));
    }

    #[test]
    fn test_url_for_v1_resource() {
        set_test_env();
        let client = Client::new_with_server_url(
            "https://api.example.com".to_string(),
            "https://candlesticks.example.com".to_string(),
        )
        .unwrap();

        let url = client.url_for_v1_resource("balances");
        assert_eq!(url, "https://api.example.com/v1/balances");

        let url = client.url_for_v1_resource("trades");
        assert_eq!(url, "https://api.example.com/v1/trades");
    }

    #[test]
    fn test_url_for_candlesticks_resource() {
        set_test_env();

        let candlestick_server_url = "https://candlesticks.example.com".to_string();

        let client = Client::new_with_server_url(
            "https://api.example.com".to_string(),
            candlestick_server_url.clone(),
        )
        .unwrap();

        let query_str = "?pair=btcusd&startTime=1000000&endTime=1000600&period=3600";

        let url = client.url_for_candlestick_resource(query_str);
        assert_eq!(
            url,
            format!("{}/candlesticks{}", candlestick_server_url, query_str)
        );
    }
}
