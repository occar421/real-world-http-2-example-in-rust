use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{:#?}", req);
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
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(route))
    });

    println!("start http listening :18888");

    let addr = SocketAddr::from(([127, 0, 0, 1], 18888));
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
