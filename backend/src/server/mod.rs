mod logger;

use self::logger::init_logger;
use crate::config::ServerConfig;
use crate::model::game::Game;
use crate::persistence::Persist;
use envconfig::Envconfig;
use log::{error, warn};
use serde::Serialize;
use std::convert::Infallible;
use std::fs;
use std::result::Result;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

const PUBLIC_PATH: &str = "/var/www/public";

pub async fn run_server() {
    let is_dev_run: bool = cfg!(debug_assertions);

    let server_config = ServerConfig::init_from_env().expect("Loading server config failed");

    init_logger(&server_config);

    let frontend_path = fs::canonicalize("../frontend")
        .map(|p| {
            p.into_os_string()
                .into_string()
                .unwrap_or_else(|_| "-".to_string())
        })
        .unwrap_or_else(|_| "-".to_string());

    let index_path: String;
    let static_path: String;
    if is_dev_run {
        warn!("Delivering development assets from {}", frontend_path);
        index_path = format!("{}/public/index.html", frontend_path);
        static_path = format!("{}/dist/", frontend_path);
    } else {
        index_path = format!("{}/index.html", PUBLIC_PATH);
        static_path = format!("{}/static/", PUBLIC_PATH);
    }

    let server_log = warp::log("server");
    let index_route = warp::get().and(warp::path::end().and(warp::fs::file(index_path)));
    let game_route = warp::path("game").and(
        warp::put()
            .map(|| {
                let new_game = Game::new("token");
                new_game.persist().expect("Creating game failed");

                warp::reply::with_status(warp::reply::json(&new_game), StatusCode::CREATED)
            })
            .recover(handle_rejection),
    );
    let api_route = warp::path("api").and(game_route);
    let static_route = warp::path("static").and(warp::fs::dir(static_path));
    let routes = index_route
        .or(static_route)
        .or(api_route)
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
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
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
