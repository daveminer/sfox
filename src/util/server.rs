use std::{
    env,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{http::Client, http::HttpVerb};
use futures_util::{SinkExt, StreamExt};
use mockito::{Mock, ServerGuard};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

pub struct ApiMock {
    pub action: HttpVerb,
    pub body: String,
    pub path: String,
    pub response_code: usize,
}

/// Start a test server configured with the provided mock and a client ready to use it.
pub async fn new_test_server_and_client(
    api_mocks: Vec<ApiMock>,
) -> (Client, mockito::ServerGuard, Vec<Mock>) {
    let (server, mocks) = start_test_http_server(api_mocks).await;

    // Use the same url for testing HTTP and candlestick servers.
    (
        Client::new_with_server_url(url(&server), url(&server)).unwrap(),
        server,
        mocks,
    )
}

pub async fn start_test_http_server(api_mocks: Vec<ApiMock>) -> (mockito::ServerGuard, Vec<Mock>) {
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

pub async fn start_test_ws_server() -> (Arc<AtomicBool>, SocketAddr, tokio::task::JoinHandle<()>) {
    // Create an Arc<AtomicBool> to share between the two threads.
    let stop = Arc::new(AtomicBool::new(false));
    let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap().clone();

    let stop_clone = Arc::clone(&stop);
    let listener_task = spawn(async move {
        while let Ok((stream, _)) = server.accept().await {
            // If the stop flag is set, break out of the loop.
            if stop_clone.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            tokio::spawn(accept_connection(stream));
        }
    });

    return (stop, addr, listener_task);
}

pub async fn stop_test_ws_server(stop: Arc<AtomicBool>) {
    stop.store(true, Ordering::Relaxed);
}

async fn accept_connection(stream: TcpStream) {
    let callback = |_req: &Request, response: Response| Ok(response);
    let mut ws_stream = accept_hdr_async(stream, callback)
        .await
        .expect("Error during the websocket handshake occurred");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await.unwrap();
        }
    }
}

fn url(server: &ServerGuard) -> String {
    format!("http://{}", server.host_with_port())
}
