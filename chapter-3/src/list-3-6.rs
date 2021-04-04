//! リスト 3-6: x-www-form-urlencoded 形式のフォームを送信する

use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values = [("test", "value")];

    let client = Client::new();
    let resp = client
        .post("http://localhost:18888")
        .form(&values)
        .send()
        .await?;

    println!("{}", resp.status());

    Ok(())
}
