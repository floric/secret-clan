pub mod app_context;
mod auth;
mod endpoints;
mod errors;
mod logger;

use self::{
    app_context::AppContext,
    endpoints::{
        games::{
            attend_game_filter, create_game_filter, get_game_filter, get_games_count_filter,
            leave_game_filter,
        },
        players::{edit_player_filter, get_player_filter, EditPlayerInput},
    },
    errors::handle_rejection,
};
use log::warn;
use std::fs;
use warp::Filter;

const PUBLIC_PATH: &str = "/var/www/public";
const AUTHORIZATION: &str = "Authorization";

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

    let game_route =
        warp::path("games").and(
            // GET /api/games
            warp::get()
                .and(warp::path::end())
                .and_then(move || async move { get_games_count_filter(ctx).await })
                .or(
                    // PUT /api/games
                    warp::put().and_then(move || async move { create_game_filter(ctx).await }),
                )
                .or(
                    // POST /api/games/attend
                    warp::post().and(warp::path!(String / "attend")).and_then(
                        move |game_token: String| async move {
                            attend_game_filter(&game_token, ctx).await
                        },
                    ),
                )
                .or(
                    // POST /api/games/leave
                    warp::post()
                        .and(warp::path!(String / "leave"))
                        .and(warp::header(AUTHORIZATION))
                        .and_then(
                            move |game_token: String, authorization: String| async move {
                                leave_game_filter(&game_token, &authorization, ctx).await
                            },
                        ),
                )
                .or(
                    // GET /api/games/:token
                    warp::get()
                        .and(warp::path!(String))
                        .and(warp::header(AUTHORIZATION))
                        .and_then(move |token: String, authorization: String| async move {
                            get_game_filter(&token, &authorization, ctx).await
                        }),
                ),
        );

    let player_route = warp::path("players").and(
        // GET /api/players/:id
        warp::get()
            .and(warp::path!(String))
            .and_then(move |id: String| async move { get_player_filter(&id, ctx).await })
            .or(
                // POST /api/players/:id
                warp::post()
                    .and(warp::path!(String))
                    .and(warp::body::json())
                    .and(warp::header(AUTHORIZATION))
                    .and_then(
                        move |id: String, input: EditPlayerInput, authorization: String| async move {
                            edit_player_filter(&id, &input, &authorization, ctx).await
                        },
                    )),
    );
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
