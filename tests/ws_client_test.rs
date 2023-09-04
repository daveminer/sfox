use sfox::websocket::{message::SubscribeMsg, Client};

#[tokio::test]
async fn test_ws_message() {
    // TODO: remove and process response msgs
    std::env::set_var("SFOX_AUTH_TOKEN", "key-goes-here");

    let ws_client = Client::new().await.unwrap();

    //let read = &ws_client.read;

    // let ethbtc_subscription_msg = SubscribeMsg {
    //     msg_type: "subscribe".to_string(),
    //     feeds: vec!["orderbook.sfox.ethbtc".to_string()],
    // };

    // let ethbtc_subscription_msg = SubscribeMsg {
    //     msg_type: "subscribe".to_string(),
    //     feeds: vec!["ticker.sfox.btcusd".to_string()],
    // };

    ws_client.authenticate().await.unwrap();

    let ethbtc_subscription_msg = SubscribeMsg {
        msg_type: "subscribe".to_string(),
        feeds: vec!["private.user.balances".to_string()],
    };

    let _msg_str: String = serde_json::to_value(ethbtc_subscription_msg)
        .unwrap()
        .to_string();

    let _ = ws_client
        .subscribe(vec!["private.user.balances".to_string()])
        .await
        .unwrap();

    // let ws_to_stdout = &read.for_each(|msg| async move {
    //     println!("MSG: {:?}", &msg);
    // });

    //ws_to_stdout;
}
