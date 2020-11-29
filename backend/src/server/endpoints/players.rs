use crate::{
    model::player::Player,
    server::{app_context::AppContext, errors::reply_with_error},
};
use warp::{hyper::StatusCode, Filter};

const PLAYERS_PATH: &str = "players";

// GET /api/players/:id
pub fn get_player(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(PLAYERS_PATH)
        .and(warp::path!(String))
        .map(move |id: String| match get_player_by_id(&ctx, &id) {
            Some(player) => warp::reply::with_status(warp::reply::json(&player), StatusCode::OK),
            None => reply_with_error(StatusCode::NOT_FOUND),
        })
}
fn get_player_by_id(ctx: &AppContext, id: &str) -> Option<Player> {
    ctx.repos().players().find_by_id(&id)
}
