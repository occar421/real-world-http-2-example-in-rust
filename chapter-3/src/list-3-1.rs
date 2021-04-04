//! リスト 3-1: GET メソッドを送信して、レスポンスのボディを画面に出力する
//! リスト 3-2: エラーチェックをしないコード
//! リスト 3-3: ステータスをコンソールに表示する

use hyper::{body, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.get("http://localhost:18888".parse()?).await?;

    let (parts, body) = resp.into_parts();
    let body = body::to_bytes(body).await?;
    println!("{}", std::str::from_utf8(&body)?);
    // 文字列で "200 OK"
    println!("Status:{}", parts.status);
    // 数値で 200
    println!("StatusCode:{}", parts.status.as_u16());
    // ヘッダーを出力
    println!("Headers:{:?}", parts.headers);
    // 特定のヘッダーを取得
    println!("Content-Length:{:?}", parts.headers["Content-Length"]);

    Ok(())
}
