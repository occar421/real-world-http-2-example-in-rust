//! リスト 3-5: HEAD メソッドを送信してヘッダーを取得する

use hyper::{Body, Client, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .request(Request::head("http://localhost:18888").body(Body::empty())?)
        .await?;

    println!("Status:{}", resp.status());
    println!("Headers:{:?}", resp.headers());

    Ok(())
}
