use crate::{
    model::player::Player,
    server::{app_context::AppContext, auth::verify_jwt_token, errors::reply_with_error},
};
use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, Filter};

const PLAYERS_PATH: &str = "players";

// GET /api/players/:id
pub fn get_player(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(PLAYERS_PATH)
        .and(warp::get())
        .and(warp::path!(String))
        .map(move |id: String| get_player_filter(&id, ctx))
}

#[derive(Deserialize)]
struct EditPlayerInput {
    name: String,
}

// POST /api/players/:id
pub fn edit_player(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(PLAYERS_PATH)
        .and(warp::post())
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(warp::header("Authorization"))
        .map(
            move |id: String, input: EditPlayerInput, authorization: String| {
                edit_player_filter(&id, &input, &authorization, ctx)
            },
        )
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

fn edit_player_filter(
    id: &str,
    input: &EditPlayerInput,
    authorization: &str,
    ctx: &AppContext,
) -> impl warp::Reply {
    verify_jwt_token(&authorization, &ctx.config().auth_secret)
        .ok()
        .and_then(|token| {
            token
                .claims()
                .get("sub")
                .map(String::from)
                .filter(|token_id| token_id == id)
                .and_then(|token_id| {
                    return match get_player_by_id(ctx, &token_id) {
                        Some(mut player) => {
                            player.set_name(&input.name);
                            ctx.repos()
                                .players()
                                .persist(&player)
                                .expect("editing player failed");
                            Some(player)
                        }
                        None => None,
                    };
                })
        })
        .map_or_else(
            || reply_with_error(StatusCode::UNAUTHORIZED),
            |player| warp::reply::with_status(warp::reply::json(&player), StatusCode::OK),
        )
}

fn get_player_by_id(ctx: &AppContext, id: &str) -> Option<Player> {
    ctx.repos().players().find_by_id(&id)
}

#[cfg(test)]
mod tests {
    use crate::{
        model::player::Player,
        server::{app_context::AppContext, auth::generate_jwt_token},
    };
    use warp::{hyper::StatusCode, Reply};

    use super::{edit_player_filter, get_player_filter, EditPlayerInput};

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
    fn should_get_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        let player_id = String::from(player.id());
        ctx.repos()
            .players()
            .persist(&player)
            .expect("Writing player failed");

        let reply = get_player_filter(&player_id, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);
    }

    #[test]
    fn should_edit_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        let player_id = String::from(player.id());
        ctx.repos()
            .players()
            .persist(&player)
            .expect("Writing player failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = edit_player_filter(
            &player_id,
            &EditPlayerInput {
                name: String::from("new name"),
            },
            &token,
            &ctx,
        );

        let updated_player = ctx
            .repos()
            .players()
            .find_by_id(player.id())
            .expect("Reading player failed");

        assert_eq!(reply.into_response().status(), StatusCode::OK);
        assert_eq!(updated_player.name(), "new name");
    }

    #[test]
    fn should_not_edit_other_player() {
        let ctx = init_ctx();

        let player = Player::new("game");
        ctx.repos()
            .players()
            .persist(&player)
            .expect("Writing player failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = edit_player_filter(
            "other",
            &EditPlayerInput {
                name: String::from("new name"),
            },
            &token,
            &ctx,
        );

        let updated_player = ctx
            .repos()
            .players()
            .find_by_id(player.id())
            .expect("Reading player failed");

        assert_eq!(reply.into_response().status(), StatusCode::UNAUTHORIZED);
        assert_eq!(updated_player.name(), player.name());
    }
}
