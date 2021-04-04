use env_logger as logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::net::SocketAddr;

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (parts, body) = req.into_parts();
    log::info!("{:?}", parts);
    let body = hyper::body::to_bytes(body).await?;
    log::info!("Body: {}", std::str::from_utf8(&body).unwrap_or_default());
    Ok(Response::new("<html><body>hello</body></html>".into()))
}

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match req.uri().path() {
        "/" => handler(req).await,
        &_ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init(); // ログ出力を見るには RUST_LOG=info 等を環境変数で指定すること

    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(route)) });

    log::info!("start http listening :18888");

    let addr = SocketAddr::from(([127, 0, 0, 1], 18888));
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
