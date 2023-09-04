use std::env;
use std::{collections::HashMap, fmt};

use futures_util::{Future, TryFutureExt};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

/// Provides candlestick chart data.
pub mod candlesticks;
/// API implementation.
pub mod v1;

#[derive(Clone, Error, Debug, Deserialize)]
pub enum HttpError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("could not create http client: {0}")]
    InitializationError(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("error while making request: {0}")]
    TransportError(String),
    #[error("could not deserialize response. Error: {0}, Response: {1}")]
    UnparseableResponse(String, String),
}

#[derive(Debug, Deserialize)]
pub struct Client {
    #[serde(skip)]
    pub auth_token: String,
    #[serde(skip)]
    pub http_client: reqwest::Client,
    pub server_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HttpVerb {
    Get,
    Post,
    Patch,
    Delete,
}

impl Into<&str> for HttpVerb {
    fn into(self) -> &'static str {
        match self {
            HttpVerb::Get => "GET",
            HttpVerb::Post => "POST",
            HttpVerb::Patch => "PATCH",
            HttpVerb::Delete => "DELETE",
        }
    }
}

pub const DEFAULT_SERVER_URL: &str = "https://api.sfox.com";

impl Client {
    /// Returns a new client with the default server URL.
    pub fn new() -> Result<Client, HttpError> {
        let server_url =
            env::var("SFOX_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string());

        build_server(server_url)
    }

    /// Builds a new client with the given server URL; useful for testing.
    pub fn new_with_server_url(server_url: String) -> Result<Client, HttpError> {
        build_server(server_url)
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
        let auth_token = self.auth_token.clone();
        println!("SELF: P{:?}", self);
        let base_response = self.action(verb.clone(), resource).bearer_auth(auth_token);

        println!("base_response: {:?}", base_response);
        let response = if Self::has_request_body(verb, &req_body) {
            base_response.json(req_body.unwrap())
        } else {
            base_response
        }
        .send()
        .map_err(|e| {
            println!("ERRRRR: {:?}", e);
            HttpError::TransportError(e.to_string())
        });

        response.and_then(|response| async move { parse_response(response).await })
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
}

fn build_server(server_url: String) -> Result<Client, HttpError> {
    println!("SERVER URL: {:?}", server_url);
    let http_client = reqwest::Client::builder()
        .build()
        .map_err(|e| HttpError::InitializationError(e.to_string()))?;

    let auth_token = env::var("SFOX_AUTH_TOKEN").map_err(|_| {
        HttpError::InitializationError("SFOX_AUTH_TOKEN env variable not set.".to_string())
    })?;

    Ok(Client {
        auth_token,
        http_client,
        server_url,
    })
}

async fn parse_response<T>(response: reqwest::Response) -> Result<T, HttpError>
where
    T: Clone + DeserializeOwned + Send + 'static,
{
    println!("RESP: {:?}", response);
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or("no text".to_string());
        return Err(HttpError::TransportError(error_text));
    }

    let text = match response.text().await {
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
    use tokio_tungstenite::tungstenite::http::Response;

    use crate::http::{parse_response, Client, HttpError, DEFAULT_SERVER_URL};
    use std::env;

    #[test]
    fn test_client_initialization() {
        let _ = env::set_var("SFOX_AUTH_TOKEN", "abc123");
        let client = Client::new().unwrap();

        // Assert auth token was set correctly
        assert_eq!(client.auth_token, "abc123");
        assert_eq!(client.server_url, DEFAULT_SERVER_URL);
    }

    #[test]
    fn test_client_initialization_with_url() {
        let _ = env::set_var("SFOX_AUTH_TOKEN", "abc123");
        let server_url = "http://localhost:4000".to_string();

        let client = Client::new_with_server_url(server_url.clone()).unwrap();

        assert_eq!(client.auth_token, "abc123");
        assert_eq!(client.server_url, server_url);
    }

    #[tokio::test]
    async fn test_parse_response_success() {
        // Create mock response
        let resp = Response::builder()
            .status(200)
            .body("\"response body\"")
            .unwrap();

        let parsed: String = parse_response(resp.into()).await.unwrap();
        assert!(!parsed.is_empty())
    }

    #[tokio::test]
    async fn test_parse_response_transport_error() {
        // Create a failed response
        let resp = Response::builder()
            .status(400)
            // Will not deserialize correctly without quotes
            .body("response body")
            .unwrap();

        // Try to parse failed response
        let result = parse_response::<String>(resp.into()).await;

        // Assert error returned
        assert!(matches!(result, Err(HttpError::TransportError(_))));
    }
}
