#[tokio::main]
async fn main() {
    let response = sfox::HttpClient::new()
        .unwrap()
        .account_balance()
        .await
        .unwrap();
    println!("Response status {}", response[0].balance);
}
