use env_logger as logger;
use log;
use std::convert::Infallible;
use warp::filters::BoxedFilter;
use warp::http::HeaderMap;
use warp::hyper::body;
use warp::{Filter, Rejection, Reply};

fn handler() -> BoxedFilter<(impl Reply,)> {
    warp::any()
        .and(warp::body::bytes())
        .map(|body: body::Bytes| {
            log::info!("{}", std::str::from_utf8(&body).unwrap());
            "<html><body>hello</body></html>"
        })
        .with(warp::log::custom(|info| {
            log::info!("{} {} {:?}", info.method(), info.path(), info.version());
            log::info!("{:#?}", info.request_headers());
        }))
        .boxed()
}

#[tokio::main]
async fn main() {
    logger::init();

    log::info!("start http listening :18888");

    warp::serve(handler()).run(([127, 0, 0, 1], 18888)).await;
}

fn log_body() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::body::bytes()
        .map(|b: body::Bytes| {
            log::info!("{}", std::str::from_utf8(&body).unwrap());
        })
        .untuple_one()
}

fn log_info() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::method().and(warp::path::param()).and(warp::query()).map(|method, query| {}).untuple_one()
}

fn log_headers() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    warp::header::headers_cloned()
        .map(|headers: HeaderMap| {
            log::info!("{:#?}", headers);
        })
        .untuple_one()
}
