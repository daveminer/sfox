use std::env;

use mockito::{Mock, ServerGuard};

use crate::{http::Client, http::HttpVerb};

pub struct ApiMock {
    pub action: HttpVerb,
    pub body: String,
    pub path: String,
    pub response_code: usize,
}

/// Start a test server configured with the provided mock and a client ready to use it.
pub async fn new_server_and_client(
    api_mocks: Vec<ApiMock>,
) -> (Client, mockito::ServerGuard, Vec<Mock>) {
    let (server, mocks) = start_test_server(api_mocks).await;

    (
        Client::new_with_server_url(url(&server)).unwrap(),
        server,
        mocks,
    )
}

pub async fn start_test_server(api_mocks: Vec<ApiMock>) -> (mockito::ServerGuard, Vec<Mock>) {
    let _ = env::set_var("SFOX_AUTH_TOKEN", "abc123");

    let mut s = mockito::Server::new_async().await;

    let mut mocks: Vec<Mock> = vec![];
    for mock in api_mocks {
        let matcher = mock.path.clone();
        let action: &str = mock.action.into();

        let mock = s
            .mock(action, matcher.as_str())
            .with_status(mock.response_code)
            .with_body(mock.body)
            .create_async()
            .await;

        mocks.push(mock);
    }

    (s, mocks)
}

fn url(server: &ServerGuard) -> String {
    format!("http://{}", server.host_with_port())
}
