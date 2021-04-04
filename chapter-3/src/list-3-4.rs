//! リスト 3-4: GET メソッドでクエリ―を送信する

use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values = [("query", "hello world")];

    let resp = reqwest::get(Url::parse_with_params("http://localhost:18888", &values)?).await?;

    println!("{}", resp.text().await?);

    Ok(())
}
