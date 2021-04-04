//! リスト 3-1: GET メソッドを送信して、レスポンスのボディを画面に出力する
//! リスト 3-2: エラーチェックをしないコード
//! リスト 3-3: ステータスをコンソールに表示する

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:18888").await?;

    // 文字列で "200 OK"
    println!("Status:{}", resp.status());
    // 数値で 200
    println!("StatusCode:{}", resp.status().as_u16());

    // ヘッダーを出力
    println!("Headers:{:?}", resp.headers());
    // 特定のヘッダーを取得
    println!("Content-Length:{:?}", resp.headers()["Content-Length"]);

    let body = resp.text().await?; // move してしまうので最後に
    println!("{}", body);

    Ok(())
}
