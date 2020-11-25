mod errors;
mod logger;

use self::{
    errors::{handle_rejection, reply_with_error},
    logger::init_logger,
};
use crate::{
    config::ServerConfig,
    logic::games::{create_new_game, get_game_by_token},
};
use envconfig::Envconfig;
use log::warn;
use std::fs;
use warp::{hyper::StatusCode, Filter};

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
    let game_route = warp::path("games").and(
        warp::put()
            .map(|| {
                let new_game = create_new_game();
                warp::reply::with_status(warp::reply::json(&new_game), StatusCode::CREATED)
            })
            .or(warp::path!(String).map(|token: String| {
                let new_game = get_game_by_token(&token);
                if new_game.is_none() {
                    return reply_with_error(StatusCode::NOT_FOUND);
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
