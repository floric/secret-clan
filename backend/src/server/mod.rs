mod logger;

use self::logger::init_logger;
use super::config::ServerConfig;
use envconfig::Envconfig;
use log::{error, warn};
use serde_derive::Serialize;
use std::convert::Infallible;
use std::fs;
use std::result::Result;
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

    let rev_dist_path = fs::canonicalize("../frontend")
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    if is_dev_run {
        warn!("Delivering development assets from {}", rev_dist_path);
    }

    let server_log = warp::log("server");
    let index_route = warp::get().and(warp::path::end().and(warp::fs::file(if is_dev_run {
        format!("{}/public/index.html", rev_dist_path)
    } else {
        String::from("/var/www/public/index.html")
    })));
    let static_route = warp::path("static").and(warp::fs::dir(format!("{}/dist", rev_dist_path)));
    let routes = index_route
        .or(static_route)
        .recover(handle_rejection)
        .with(server_log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], server_config.port))
        .await
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = "UNHANDLED_REJECTION";

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        error!("Unexpected error: {:?}", err);
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
