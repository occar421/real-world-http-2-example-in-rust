//! リスト 3-5: HEAD メソッドを送信してヘッダーを取得する

use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.head("http://localhost:18888").send().await?;

    println!("Status:{}", resp.status());
    println!("Headers:{:?}", resp.headers());

    Ok(())
}
