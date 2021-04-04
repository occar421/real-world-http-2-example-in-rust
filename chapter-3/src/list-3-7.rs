//! リスト 3-7: POST メソッドで任意のボディを送信
//! リスト 3-8: 文字列を io.Reader インターフェース化する

use reqwest::{multipart, Client};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./chapter-3/src/list-3-7.rs");
    // ※ リスト 3-8 では単に文字列リテラルを指定する
    let file_content = fs::read_to_string(file_path).await?;

    let form = multipart::Form::new().part(
        "file",
        multipart::Part::text(file_content)
            .file_name(file_path.file_name().map(|n| n.to_str()).flatten().unwrap()),
    );

    let client = Client::new();
    let resp = client
        .post("http://localhost:18888")
        .multipart(form)
        .send()
        .await?;

    println!("{}", resp.status());

    Ok(())
}
