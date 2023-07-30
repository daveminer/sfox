use sfox::http::{v1::volume::Interval, SFox};

#[tokio::test]
async fn test_account_balance() {
    let client = setup().await;
    let response = client.account_balance().await;
    println!("Account balance response: {:?}", response);
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_transaction_history() {
    let client = setup().await;
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
    let client = setup().await;
    let response = client.crypto_deposit_address("btc").await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_new_crypto_deposit_address() {
    let client = setup().await;
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
    let client = setup().await;
    let response = client.withdraw_fee("btc").await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_fees() {
    let client = setup().await;
    let response = client.fees().await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_currencies() {
    let client = setup().await;
    let response = client.currencies().await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_currency_pairs() {
    let client = setup().await;
    let response = client.currency_pairs().await;
    println!("PAIRS: {:?}", response);
    //assert!(response.is_ok());
}

// Untested
//
// #[tokio::test]
// async fn test_place_order() {
//     let client = Client::new().unwrap();

//     let response = SFox::place_order(
//             "buy",
//             "btcusd",
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
//     let response = SFox::cancel_order("123").await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_cancel_order() {
//     let response = SFox::cancel_orders(vec![123, 456]).await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_cancel_all_orders() {
//     let response = SFox::cancel_all_orders().await;
//     assert!(response.is_ok());
// }

#[tokio::test]
async fn test_request_for_quote() {
    let client = setup().await;

    let response = client
        .request_for_quote("btcusd", "buy", Some(1.001), None, None)
        .await;
    println!("{:?}", response);
    assert!(response.is_ok());
}

// Untested

// #[tokio::test]
// async fn test_execute_order_on_quote() {
//     let response = SFox::request_for_quote("btcusd", "buy", Some(1.001), None, None)
//         .await;
//     println!("{:?}", response);
//     assert!(response.is_ok());
// }

#[tokio::test]
async fn test_candlesticks() {
    let client = setup().await;

    let response = client
        .candlesticks("btcusd", 1690477895, 1690564295, 600)
        .await;

    assert!(response.is_ok());
}

//TODO: test response shape for flags
#[tokio::test]
async fn test_volume() {
    let client = setup().await;

    let response = client
        .volume(
            1690477895000,
            1690564295000,
            Interval::Hour,
            "btc",
            true,
            true,
        )
        .await;

    println!("RESP: {:?}", response);
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_order_estimate() {
    let client = setup().await;

    let response = client
        .order_estimate("buy", "btcusd", 1.01, 35000.05, "Smart")
        .await;

    println!("REPPP: {:?}", response.unwrap());

    //assert!(response.is_ok());
}

#[tokio::test]
async fn test_order_book() {
    let client = setup().await;

    let response = client.order_book("btcusd").await;

    println!("REPPP: {:?}", response.unwrap());

    //assert!(response.is_ok());
}

// Untested
// #[tokio::test]
// async fn test_post_trade_settlement() {
//     let client = setup().await;

//     let response = client.post_trade_settlement().await;

//     println!("REPPP: {:?}", response.unwrap());

//     //assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_post_trade_settlement_positions() {
//     let client = setup().await;

//     let response = client.post_trade_settlement_positions(None).await;

//     println!("REPPP: {:?}", response.unwrap());

//     //assert!(response.is_ok());
// }

#[tokio::test]
async fn test_post_trade_settlement_interest() {
    let client = setup().await;

    let response = client.post_trade_settlement_interest().await;
    assert!(response.is_ok());
}

// Untested
// #[tokio::test]
// async fn test_wallet_transfer() {
//     let client = setup().await;

//     let response = client
//         .wallet_transfer("btc".into(), 0.001, "0x00".into(), "0x00".into())
//         .await;
//     assert!(response.is_ok());
// }

// Untested
// #[tokio::test]
// async fn test_loan_metrics() {
//     let client = setup().await;

//     let response = client.loan_metrics().await;

//     assert!(response.is_ok());
// }

#[tokio::test]
async fn test_loan_positions() {
    let client = setup().await;

    let response = client.loan_positions(Some("active".to_string())).await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_custody_addresses() {
    let client = setup().await;

    let response = client.custody_addresses().await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_add_custody_address() {
    let client = setup().await;

    let response = client
        .add_custody_address(
            "test alias".to_string(),
            "btc".to_string(),
            "OxOO".to_string(),
        )
        .await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_approval_rules() {
    let client = setup().await;

    let response = client.approval_rules().await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_add_approval_rule() {
    let client = setup().await;

    let response = client.add_approval_rule("WITHDRAW".to_string(), 2, 2).await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_edit_approval_rule() {
    let client = setup().await;

    let response = client.edit_approval_rule(1, 2, 0.01).await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_approval_requests() {
    let client = setup().await;

    let response = client.approval_requests(Some(true)).await;

    assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_respond_to_approval_request() {
    let client = setup().await;

    let response = client.respond_to_approval_request(1, true).await;

    assert!(response.is_ok());
}

#[tokio::test]
async fn test_staking_currencies() {
    let client = setup().await;

    let response = client.staking_currencies().await;

    assert!(response.is_ok())
}

// Untested
#[tokio::test]
async fn test_staking_transactions() {
    let client = setup().await;

    let response = client.staking_transactions().await;

    println!("REPPP: {:?}", response.unwrap());

    //assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_stake() {
    let client = setup().await;

    let response = client.stake("avax".to_string(), 0.1).await;

    println!("REPPP: {:?}", response.unwrap());

    //assert!(response.is_ok());
}

// Untested
#[tokio::test]
async fn test_unstake() {
    let client = setup().await;

    let response = client.unstake("avax".to_string(), 0.1).await;

    println!("REPPP: {:?}", response.unwrap());

    //assert!(response.is_ok());
}

async fn setup() -> SFox {
    std::env::set_var(
        "SFOX_AUTH_TOKEN",
        "29cfe5399d9a5f09abd6d48382245c382862b8a462a2d391a54166d54d6915ff",
    );

    return SFox::new(None).unwrap();
}
