use crate::{
    model::player::Player,
    server::{app_context::AppContext, errors::reply_with_error},
};
use serde::Serialize;
use warp::{hyper::StatusCode, Filter};

const PLAYERS_PATH: &str = "players";

// GET /api/players/:id
pub fn get_player(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(PLAYERS_PATH)
        .and(warp::path!(String))
        .map(move |id: String| get_player_filter(&id, ctx))
}

fn get_player_filter(id: &str, ctx: &AppContext) -> impl warp::Reply {
    #[derive(Serialize)]
    struct GetPlayerResponse {
        id: String,
        name: String,
    }

    match get_player_by_id(ctx, &id) {
        Some(player) => warp::reply::with_status(
            warp::reply::json(&GetPlayerResponse {
                id: String::from(player.id()),
                name: String::from(player.name()),
            }),
            StatusCode::OK,
        ),
        None => reply_with_error(StatusCode::NOT_FOUND),
    }
}
fn get_player_by_id(ctx: &AppContext, id: &str) -> Option<Player> {
    ctx.repos().players().find_by_id(&id)
}

#[cfg(test)]
mod tests {
    use crate::{model::player::Player, server::app_context::AppContext};
    use warp::{hyper::StatusCode, Reply};

    use super::get_player_filter;

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[test]
    fn should_not_get_unknown_player() {
        let ctx = init_ctx();

        let reply = get_player_filter("unknown", &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn should_get_unknown_player() {
        let ctx = init_ctx();

        let player = Player::new("player", "game");
        let player_id = String::from(player.id());
        ctx.repos()
            .players()
            .persist(player)
            .expect("Writing player failed");

        let reply = get_player_filter(&player_id, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);
    }
}
