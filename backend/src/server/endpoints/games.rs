use crate::{
    model::{game::Game, player::Player},
    server::{
        app_context::AppContext, auth::generate_jwt_token, auth::verify_jwt_token,
        errors::reply_with_error,
    },
};
use log::debug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use std::iter;
use warp::{hyper::StatusCode, Filter};

const GAMES_PATH: &str = "games";

// this value determines the findability of a game and is a tradeoff between security and user friendliness
// 5 tokens mean a chance of finding a random game of 1:60466176.
const TOKEN_CHARS_COUNT: usize = 5;

// GET /api/games/:token
pub fn get_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::get())
        .and(warp::path!(String))
        .and(warp::header("Authorization"))
        .map(move |token: String, authorization: String| {
            get_game_filter(&token, &authorization, ctx)
        })
}

// GET /api/games/
pub fn get_games_count(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::get())
        .and(warp::path::end())
        .map(move || {
            #[derive(Serialize)]
            struct GetGamesResponse {
                total: usize,
            };

            warp::reply::with_status(
                warp::reply::json(&GetGamesResponse {
                    total: ctx.repos().games().total_count(),
                }),
                StatusCode::OK,
            )
        })
}

// PUT /api/games/
pub fn create_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::put())
        .map(move || create_game_filter(ctx))
}

// POST /api/games/attend
pub fn attend_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::post())
        .and(warp::path!(String / "attend"))
        .map(move |game_token: String| attend_game_filter(&game_token, ctx))
}

// POST /api/games/leave
pub fn leave_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::post())
        .and(warp::path!(String / "leave"))
        .and(warp::header("Authorization"))
        .map(move |game_token: String, authorization: String| {
            leave_game_filter(&game_token, &authorization, ctx)
        })
}

fn get_game_filter(game_token: &str, authorization: &str, ctx: &AppContext) -> impl warp::Reply {
    verify_jwt_token(authorization, &ctx.config().auth_secret)
        .ok()
        .and_then(|token| token.claims().get("sub").map(String::from))
        .map_or_else(
            || reply_with_error(StatusCode::UNAUTHORIZED),
            |token| {
                get_game_by_token(ctx, game_token).map_or_else(
                    || reply_with_error(StatusCode::NOT_FOUND),
                    |game| {
                        ctx.repos()
                            .players()
                            .find_by_id(&token)
                            .filter(|player| {
                                game.player_ids().contains(player.id())
                                    || game.admin_id() == player.id()
                            })
                            .map_or_else(
                                || reply_with_error(StatusCode::NOT_FOUND),
                                |mut player| {
                                    player.heartbeat();
                                    ctx.repos()
                                        .players()
                                        .persist(&player)
                                        .expect("Persisting heartbeat has failed");
                                    warp::reply::with_status(
                                        warp::reply::json(&game),
                                        StatusCode::OK,
                                    )
                                },
                            )
                    },
                )
            },
        )
}

fn create_game_filter(ctx: &AppContext) -> impl warp::Reply {
    fn generate_game_token() -> String {
        let mut rng = thread_rng();

        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase())
            .take(TOKEN_CHARS_COUNT)
            .collect()
    }

    let game_token = generate_game_token();
    let player = create_new_player(&game_token, ctx);
    let new_game = create_new_game(player.id(), &game_token, ctx);

    #[derive(Serialize)]
    struct CreateGameReponse {
        game: Game,
        admin: Player,
    };

    warp::reply::with_status(
        warp::reply::json(&CreateGameReponse {
            game: new_game,
            admin: player,
        }),
        StatusCode::CREATED,
    )
}

fn attend_game_filter(game_token: &str, ctx: &AppContext) -> impl warp::Reply {
    match get_game_by_token(ctx, &game_token) {
        Some(mut game) => {
            let player = create_new_player(&game_token, ctx);

            game.add_player(player.id());
            ctx.repos()
                .games()
                .persist(&game)
                .expect("Adding player failed");

            warp::reply::with_status(warp::reply::json(&player), StatusCode::OK)
        }
        None => reply_with_error(StatusCode::NOT_FOUND),
    }
}

fn leave_game_filter(game_token: &str, authorization: &str, ctx: &AppContext) -> impl warp::Reply {
    match get_game_by_token(ctx, &game_token) {
        Some(mut game) => verify_jwt_token(&authorization, &ctx.config().auth_secret)
            .map_or(None, |token| token.claims().get("sub").map(String::from))
            .map_or_else(
                || reply_with_error(StatusCode::UNAUTHORIZED),
                |token_id| {
                    game.remove_player(&token_id);
                    match ctx.repos().games().persist(&game) {
                        Ok(_) => reply_with_error(StatusCode::OK),
                        Err(_) => reply_with_error(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                },
            ),
        None => reply_with_error(StatusCode::NOT_FOUND),
    }
}

fn create_new_game(admin_id: &str, token: &str, ctx: &AppContext) -> Game {
    let new_game = Game::new(admin_id, token);
    let new_token = new_game.token();
    ctx.repos()
        .games()
        .persist(&new_game)
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);

    new_game
}

fn create_new_player(game_token: &str, ctx: &AppContext) -> Player {
    let mut player = Player::new(game_token);
    let user_token = generate_jwt_token(&player, &ctx.config().auth_secret);
    player.update_token(&user_token);

    ctx.repos()
        .players()
        .persist(&player)
        .expect("Creating player failed");
    debug!("Created player with token {}", player.id());

    player
}

fn get_game_by_token(ctx: &AppContext, token: &str) -> Option<Game> {
    ctx.repos().games().find_by_id(&token.to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::{attend_game_filter, create_game_filter, get_game_filter, leave_game_filter};
    use crate::{
        model::{game::Game, player::Player},
        server::{app_context::AppContext, auth::generate_jwt_token},
    };
    use warp::{hyper::StatusCode, Reply};

    const GAME_TOKEN: &str = "ACDEF";

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[test]
    fn should_not_get_game_unauthorized() {
        let ctx = init_ctx();

        let reply = get_game_filter("invalid", "auth", &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn should_not_get_unknown_game() {
        let ctx = init_ctx();

        let token = generate_jwt_token(&Player::new("game"), &ctx.config().auth_secret);

        let reply = get_game_filter("game", &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn should_get_game_for_admin() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        ctx.repos()
            .players()
            .persist(&admin)
            .expect("Writing player failed");
        ctx.repos()
            .games()
            .persist(&Game::new(admin.id(), GAME_TOKEN))
            .expect("Writing game failed");

        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);
    }

    #[test]
    fn should_get_game_for_player() {
        let ctx = init_ctx();

        let player = Player::new(GAME_TOKEN);
        ctx.repos()
            .players()
            .persist(&player)
            .expect("Writing player failed");
        let mut game = Game::new("admin", GAME_TOKEN);
        game.add_player(player.id());
        ctx.repos()
            .games()
            .persist(&game)
            .expect("Writing game failed");

        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);
    }

    #[test]
    fn should_get_game_and_send_heartbeat() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        let first_time = admin.last_action_time();
        ctx.repos()
            .players()
            .persist(&admin)
            .expect("Writing admin failed");

        ctx.repos()
            .games()
            .persist(&Game::new(admin.id(), GAME_TOKEN))
            .expect("Writing game failed");

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);

        let updated_admin = ctx
            .repos()
            .players()
            .find_by_id(admin.id())
            .expect("Reading admin failed");
        assert!(updated_admin.last_action_time().gt(&first_time));
    }

    #[test]
    fn should_create_new_game() {
        let ctx = init_ctx();

        let reply = create_game_filter(&ctx);

        assert_eq!(reply.into_response().status(), StatusCode::CREATED);
    }

    #[test]
    fn should_not_attend_unknown_game() {
        let ctx = init_ctx();

        let reply = attend_game_filter("test", &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn should_attend_game() {
        let ctx = init_ctx();

        ctx.repos()
            .games()
            .persist(&Game::new("admin", GAME_TOKEN))
            .expect("Writing game failed");

        let reply = attend_game_filter(GAME_TOKEN, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);
    }

    #[test]
    fn should_leave_game() {
        let ctx = init_ctx();

        let mut game = Game::new("admin", GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);
        game.add_player(player.id());
        ctx.repos()
            .games()
            .persist(&game)
            .expect("Writing game failed");

        assert!(game.player_ids().contains(player.id()));

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .repos()
            .games()
            .find_by_id(GAME_TOKEN)
            .expect("Couldnt find game");
        assert!(!updated_game.player_ids().contains(player.id()));
    }

    #[test]
    fn should_leave_game_and_select_new_admin() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        let mut game = Game::new(admin.id(), GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        game.add_player(player.id());
        ctx.repos()
            .games()
            .persist(&game)
            .expect("Writing game failed");

        assert_eq!(game.admin_id(), admin.id());

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx);

        assert_eq!(reply.into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .repos()
            .games()
            .find_by_id(GAME_TOKEN)
            .expect("Couldnt find game");
        assert!(updated_game.player_ids().is_empty());
        assert_eq!(updated_game.admin_id(), player.id());
    }
}
