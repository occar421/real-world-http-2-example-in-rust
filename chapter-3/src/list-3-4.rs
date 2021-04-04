//! リスト 3-4: GET メソッドでクエリ―を送信する

use hyper::{body, Client};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values = [("query", "hello world")];

    let client = Client::new();
    let resp = client
        .get(
            Url::parse_with_params("http://localhost:18888", &values)?
                .into_string()
                .parse()?,
        )
        .await?;

    let body = body::to_bytes(resp.into_body()).await?;
    println!("{}", std::str::from_utf8(&body)?);

    Ok(())
}
