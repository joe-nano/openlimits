use dotenv::dotenv;
use rust_decimal::prelude::Decimal;
use std::env;

use openlimits::binance::{
    model::{AllOrderReq, TradeHistoryReq},
    Binance,
};

#[tokio::test]
async fn get_account() {
    let exchange = init().await;
    let resp = exchange.get_account().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_balance() {
    let exchange = init().await;
    let resp = exchange.get_balance("BTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_open_orders() {
    let exchange = init().await;
    let resp = exchange.get_open_orders("BNBBTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init().await;
    let resp = exchange.get_all_open_orders().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders() {
    let exchange = init().await;
    let params = AllOrderReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange.get_all_orders(&params).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order() {
    let exchange = init().await;
    let transaction = exchange
        .limit_sell("BNBBTC", Decimal::new(1, 1), Decimal::new(2, 3))
        .await
        .unwrap();
    let resp = exchange
        .get_order("BNBBTC", transaction.order_id)
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let resp = exchange
        .limit_buy("BNBBTC", Decimal::new(1, 1), Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn rounded_limit_buy() {
    let exchange = init().await;

    let resp = exchange
        .limit_buy("BNBBTC", Decimal::new(12345678, 8), Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let resp = exchange
        .limit_sell("BNBBTC", Decimal::new(1, 1), Decimal::new(2, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let resp = exchange
        .market_buy("BNBBTC", Decimal::new(1, 0))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let resp = exchange
        .market_sell("BNBBTC", Decimal::new(1, 0))
        .await
        .unwrap();

    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let transaction = exchange
        .limit_sell("BNBBTC", Decimal::new(1, 1), Decimal::new(2, 3))
        .await
        .unwrap();
    let resp = exchange
        .cancel_order("BNBBTC", transaction.order_id)
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn trade_history() {
    let exchange = init().await;
    let params = TradeHistoryReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };

    let resp = exchange.trade_history(&params).await.unwrap();
    println!("{:?}", resp);
}

async fn init() -> Binance {
    dotenv().ok();
    Binance::with_credential(
        &env::var("BINANCE_API_KEY").unwrap(),
        &env::var("BINANCE_API_SECRET").unwrap(),
        true,
    )
    .await
}
