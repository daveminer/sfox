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

// Untested
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

// Untested
//
// #[tokio::test]
// async fn test_withdraw() {
//     let client = Client::new().unwrap();

//     let response = client.withdraw("0x00", 0.000001, "btc", false).await;
//     assert!(response.is_ok());
// }

#[tokio::test]
async fn test_withdrawal_fee() {
    let client = Client::new().unwrap();

    let response = client.withdraw_fee("btc").await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_fees() {
    let client = Client::new().unwrap();

    let response = client.fees().await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_currencies() {
    let client = Client::new().unwrap();

    let response = client.currencies().await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_currency_pairs() {
    let client = Client::new().unwrap();

    let response = client.currency_pairs().await;
    println!("PAIRS: {:?}", response);
    //assert!(response.is_ok());
}

// Untested
//
// #[tokio::test]
// async fn test_place_order() {
//     let client = Client::new().unwrap();

//     let response = client
//         .place_order(
//             "buy",
//             "ETH/USDT",
//             1994.72,
//             0.0003,
//             "NetPrice",
//             200,
//             Some("test-client-id"),
//         )
//         .await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_cancel_order() {
//     let client = Client::new().unwrap();

//     let response = client.cancel_order("123").await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_cancel_order() {
//     let client = Client::new().unwrap();

//     let response = client.cancel_orders(vec![123, 456]).await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_cancel_all_orders() {
//     let client = Client::new().unwrap();

//     let response = client.cancel_all_orders().await;
//     assert!(response.is_ok());
// }

#[tokio::test]
async fn test_request_for_quote() {
    let client = Client::new().unwrap();

    let response = client
        .request_for_quote("btcusd", "buy", Some(1.001), None, None)
        .await;
    println!("{:?}", response);
    assert!(response.is_ok());
}

// Untested

// #[tokio::test]
// async fn test_execute_order_on_quote() {
//     let client = Client::new().unwrap();

//     let response = client
//         .request_for_quote("btcusd", "buy", Some(1.001), None, None)
//         .await;
//     println!("{:?}", response);
//     assert!(response.is_ok());
// }
