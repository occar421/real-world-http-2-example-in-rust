use hyper::{body, Client};

// リスト 3-1 3-2 3-3

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.get("http://localhost:18888".parse()?).await?;

    let status = resp.status();
    let headers = resp.headers().clone();
    let body = body::to_bytes(resp.into_body()).await?;
    println!("{}", std::str::from_utf8(&body)?);
    // 文字列で "200 OK"
    println!("Status:{}", status);
    // 数値で 200
    println!("StatusCode:{}", status.as_u16());
    // ヘッダーを出力
    println!("Headers:{:?}", headers);
    // 特定のヘッダーを取得
    println!("Content-Length:{:?}", headers["Content-Length"]);

    Ok(())
}
