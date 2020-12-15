pub mod app_context;
mod auth;
mod endpoints;
mod errors;
mod logger;

use self::{
    app_context::AppContext,
    endpoints::games::get_game,
    endpoints::games::get_games_count,
    endpoints::{
        games::{attend_game, create_game, leave_game},
        players::get_player,
    },
    errors::handle_rejection,
};
use endpoints::players::edit_player;
use log::warn;
use std::fs;
use warp::Filter;

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

    let game_route = get_games_count(ctx)
        .or(create_game(ctx))
        .or(attend_game(ctx))
        .or(leave_game(ctx))
        .or(get_game(ctx));
    let player_route = get_player(ctx).or(edit_player(ctx));
    let api_route = warp::path("api").and(game_route.or(player_route));

    let static_route = warp::path("static").and(warp::fs::dir(static_path));
    let index_route = warp::get().and(warp::path::end().and(warp::fs::file(index_path)));

    let routes = index_route
        .or(static_route)
        .or(api_route)
        .recover(handle_rejection)
        .with(warp::log("server"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], ctx.config().port))
        .await;
}
