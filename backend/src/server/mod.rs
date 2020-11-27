pub mod app_context;
mod errors;
mod logger;

use self::{
    app_context::AppContext,
    errors::{handle_rejection, reply_with_error},
};
use crate::logic::games::{create_new_game, get_game_by_token};
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;
use warp::{hyper::StatusCode, Filter};

const PUBLIC_PATH: &str = "/var/www/public";

pub async fn run_server(ctx: &'static AppContext) {
    let frontend_path = fs::canonicalize("../frontend")
        .map(|p| {
            p.into_os_string()
                .into_string()
                .unwrap_or_else(|_| "-".to_string())
        })
        .unwrap_or_else(|_| "-".to_string());

    let index_path: String;
    let static_path: String;
    if ctx.is_dev() {
        warn!("Delivering development assets from {}", frontend_path);
        index_path = format!("{}/public/index.html", frontend_path);
        static_path = format!("{}/dist/", frontend_path);
    } else {
        index_path = format!("{}/index.html", PUBLIC_PATH);
        static_path = format!("{}/static/", PUBLIC_PATH);
    }

    let index_route = warp::get().and(warp::path::end().and(warp::fs::file(index_path)));
    let game_route = warp::path("games").and(
        warp::get()
            .and(warp::path::end())
            .map(move || {
                #[derive(Serialize, Deserialize)]
                struct GamesSummary {
                    total: usize,
                };

                warp::reply::with_status(
                    warp::reply::json(&GamesSummary {
                        total: ctx.repos().games().total_count(),
                    }),
                    StatusCode::OK,
                )
            })
            .or(warp::put().map(move || {
                let new_game = create_new_game(&ctx);
                warp::reply::with_status(warp::reply::json(&new_game), StatusCode::CREATED)
            }))
            .or(warp::path!(String).map(move |token: String| {
                let new_game = get_game_by_token(&ctx, &token);
                if new_game.is_none() {
                    return reply_with_error(StatusCode::NOT_FOUND);
                }

                warp::reply::with_status(warp::reply::json(&new_game.unwrap()), StatusCode::OK)
            })),
    );
    let api_route = warp::path("api").and(game_route);
    let static_route = warp::path("static").and(warp::fs::dir(static_path));
    let routes = index_route
        .or(static_route)
        .or(api_route)
        .recover(handle_rejection)
        .with(warp::log("server"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], ctx.config().port))
        .await;
}
