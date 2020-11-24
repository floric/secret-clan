mod logger;

use self::logger::init_logger;
use crate::config::ServerConfig;
use crate::model::game::Game;
use crate::persistence::Persist;
use envconfig::Envconfig;
use log::{debug, error, info, warn};
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

fn build_error_404() -> ErrorMessage {
    ErrorMessage {
        code: StatusCode::NOT_FOUND.as_u16(),
        message: "NOT_FOUND".to_string(),
    }
}

fn build_error_500() -> ErrorMessage {
    ErrorMessage {
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        message: "INTERNAL_SERVER_ERROR".to_string(),
    }
}

fn build_error_401() -> ErrorMessage {
    ErrorMessage {
        code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
        message: "METHOD_NOT_ALLOWED".to_string(),
    }
}

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
    let game_route = warp::path("games").and(
        warp::put()
            .map(|| {
                let new_game = Game::new();
                new_game.persist().expect("Creating game failed");
                debug!("Created game with token {}", new_game.token());
                warp::reply::with_status(warp::reply::json(&new_game), StatusCode::CREATED)
            })
            .or(warp::path!(String).map(|token_id: String| {
                let new_game = Game::new().find_by_id(&token_id.to_uppercase());
                if new_game.is_none() {
                    return warp::reply::with_status(
                        warp::reply::json(&build_error_404()),
                        StatusCode::NOT_FOUND,
                    );
                }

                warp::reply::with_status(warp::reply::json(&new_game), StatusCode::OK)
            })),
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
    let mut err_msg: ErrorMessage = build_error_500();

    if err.is_not_found() {
        err_msg = build_error_404();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        err_msg = build_error_401();
    } else {
        error!("Unexpected error: {:?}", err);
    }

    let json = warp::reply::json(&err_msg);

    Ok(warp::reply::with_status(
        json,
        StatusCode::from_u16(err_msg.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    ))
}
