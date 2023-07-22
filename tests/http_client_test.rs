use sfox::http::Client;

#[tokio::test]
async fn test_account_balance() {
    let client = Client::new().unwrap();

    let response = client.account_balance().await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_transaction_history() {
    let client = Client::new().unwrap();

    let response = client.transaction_history().await;
    assert!(response.is_ok());
}

// Untested.
//
// #[tokio::test]
// async fn test_ach_bank_transfer() {
//     let client = Client::new().unwrap();

//     let response = client.ach_bank_transfer().await;
//     assert!(response.is_ok());
// }

// Test populated case
#[tokio::test]
async fn test_crypto_deposit_address() {
    let client = Client::new().unwrap();

    let response = client.crypto_deposit_address("btc").await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_new_crypto_deposit_address() {
    let client = Client::new().unwrap();

    let response = client.new_crypto_deposit_address("btc").await;
    assert!(response.is_ok());
}
