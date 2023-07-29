use std::collections::HashMap;
use std::env;

use futures_util::{Future, TryFutureExt};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use thiserror::Error;

pub mod chart;
pub mod v1;

#[derive(Clone, Error, Debug, Deserialize)]
pub enum HttpError {
    #[error("could not create http client: {0}")]
    InitializationError(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("error while making http request: {0}")]
    TransportError(String),
    #[error("could not deserialize response. Error: {0}, Response: {1}")]
    UnparseableResponse(String, String),
}

#[derive(Debug, Deserialize)]
pub struct SFox {
    #[serde(skip)]
    pub auth_token: String,
    #[serde(skip)]
    pub http_client: reqwest::Client,
    pub server_url: String,
}

#[derive(Clone)]
pub enum HttpVerb {
    Get,
    Post,
    Patch,
    Delete,
}

pub const DEFAULT_SERVER_URL: &str = "https://api.sfox.com";

impl SFox {
    pub fn new(server_url: Option<&str>) -> Result<SFox, HttpError> {
        let http_client = reqwest::Client::builder()
            .build()
            .map_err(|e| HttpError::InitializationError(e.to_string()))?;

        let auth_token = env::var("SFOX_AUTH_TOKEN").map_err(|_| {
            HttpError::InitializationError("SFOX_AUTH_TOKEN env variable not set.".to_string())
        })?;

        let server_url = match server_url {
            Some(url) => url.into(),
            None => env::var("SFOX_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string()),
        };

        println!("SERVER_URL: {}", server_url);

        Ok(SFox {
            auth_token,
            http_client,
            server_url,
        })
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
        let base_response = self.action(verb.clone(), &resource).bearer_auth(auth_token);

        let response = if Self::has_request_body(verb, &req_body) {
            base_response.json(req_body.unwrap())
        } else {
            base_response
        }
        .send()
        .map_err(|e| HttpError::TransportError(e.to_string()));

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

    fn url_from_resource(&self, resource: &str) -> String {
        format!("{}/{}", self.server_url, resource)
    }
}

async fn parse_response<T>(response: reqwest::Response) -> Result<T, HttpError>
where
    T: Clone + DeserializeOwned + Send + 'static,
{
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
