use std::collections::HashMap;

use futures_util::{Future, TryFutureExt};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use thiserror::Error;

use crate::settings::Settings;

pub mod resources;

// #[derive(Clone, Deserialize)]
// pub enum ApiResponse<T> {
//     Single(T),
//     Multiple(Vec<T>),
// }
// trait IntoVec {}
// trait IntoSingle {}

// impl<T> IntoVec for ApiResponse<T> {}
// impl<T> IntoSingle for ApiResponse<T> {}

// impl<T: IntoVec> Into<Vec<T>> for ApiResponse<T> {
//     // Vec<T> impl
// }

// impl<T: IntoSingle> Into<T> for ApiResponse<T> {
//     // T impl
// }

// impl<T> ApiResponse<T> {
//     pub fn into_vec(self) -> Vec<T> {
//         // existing implementation to return Vec<T>
//     }

//     pub fn into_single(self) -> T {
//         // existing implementation to return T
//     }
// }

// impl<T> Into<Vec<T>> for ApiResponse<T> {
//     fn into(self) -> Vec<T> {
//         self.into_vec()
//     }
// }

// impl<T> Into<T> for ApiResponse<T> {
//     fn into(self) -> T {
//         self.into_single()
//     }
// }

#[derive(Clone, Error, Debug, Deserialize)]
pub enum HttpError {
    #[error("could not create http client: {0}")]
    InitializationError(String),
    #[error("error while making http request: {0}")]
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

impl Client {
    pub fn new() -> Result<Client, HttpError> {
        let settings = Settings::new().unwrap();

        let http_client = reqwest::Client::builder()
            .build()
            .map_err(|e| HttpError::InitializationError(e.to_string()))?;

        let auth_token = settings.auth.api_key;
        let server_url = settings.http.server_url;

        Ok(Client {
            auth_token,
            http_client,
            server_url,
        })
    }

    // pub fn get_currencies(self) -> ResponseFuture {
    //     self.get_request("currency")
    // }

    // pub fn get_fees(self) -> ResponseFuture {
    //     self.get_request("markets/currency-pairs")
    // }

    // pub fn withdrawal_fee(self, currency: &str) -> ResponseFuture {
    //     self.get_request(&format!("withdraw-fee/{}", currency))
    // }

    // Modify this function so it can return a struct or an array of structs
    fn get_request<T>(self, resource: &str) -> impl Future<Output = Result<T, HttpError>>
    where
        T: Clone + DeserializeOwned + Send + 'static,
    {
        // Use request to make a request
        let url = format!("{}/v1/{}", self.server_url, resource);

        let response = self
            .http_client
            .get(url)
            .bearer_auth(self.auth_token)
            .send()
            .map_err(|e| HttpError::TransportError(e.to_string()));

        response.and_then(|response| async move { parse_response(response).await })
    }

    fn post_request<T>(
        self,
        resource: &str,
        req_body: &HashMap<String, String>,
    ) -> impl Future<Output = Result<T, HttpError>>
    where
        T: Clone + DeserializeOwned + Send + 'static,
    {
        let url = format!("{}/v1/{}", self.server_url, resource);

        let base_resp = self.http_client.post(url).bearer_auth(self.auth_token);

        let response = if req_body.is_empty() {
            base_resp
        } else {
            base_resp.json(req_body)
        }
        .send()
        .map_err(|e| HttpError::TransportError(e.to_string()));

        response.and_then(|response| async move { parse_response(response).await })
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
