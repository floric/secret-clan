mod logger;

use std::convert::Infallible;

use self::logger::init_logger;
use super::config::ServerConfig;
use envconfig::Envconfig;
use log::error;
use serde_derive::Serialize;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn run_server() {
    let is_dev_run: bool = cfg!(debug_assertions);

    let server_config = ServerConfig::init_from_env().expect("Loading server config failed");

    init_logger(&server_config);

    let index_route = warp::get().and(warp::path::end().and(warp::fs::file(if is_dev_run {
        "/var/www/public/index.html"
    } else {
        "/var/www/public/index.html"
    })));
    let static_route = warp::path("static").and(warp::fs::dir("/var/www/public/static"));
    let routes = index_route.or(static_route).recover(handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], server_config.port))
        .await
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        error!("Unexpected error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
