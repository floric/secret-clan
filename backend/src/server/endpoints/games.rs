use crate::{
    model::{Game, Player},
    server::{
        app_context::AppContext,
        auth::extract_verified_id,
        auth::generate_jwt_token,
        reply::{reply_error, reply_error_with_details, reply_success},
    },
};
use log::debug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use std::{convert::Infallible, iter};
use warp::hyper::StatusCode;

// this value determines the findability of a game and is a tradeoff between security and user friendliness
// 5 tokens mean a chance of finding a random game of 1:60466176.
const TOKEN_CHARS_COUNT: usize = 5;

pub async fn get_game_filter(
    game_token: &str,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match extract_verified_id(authorization, ctx) {
        Some(token) => match ctx
            .db()
            .games()
            .get(game_token)
            .await
            .expect("Reading game has failed")
        {
            Some(mut game) => {
                match ctx
                    .db()
                    .players()
                    .get(&token)
                    .await
                    .expect("Reading player has failed")
                    .filter(|player| {
                        game.player_ids().contains(player.id())
                            || game.admin_id().is_some()
                                && game.admin_id().as_ref().unwrap() == player.id()
                    }) {
                    Some(mut player) => {
                        player.heartbeat();
                        ctx.db()
                            .players()
                            .persist(&player)
                            .await
                            .expect("Persisting heartbeat has failed");
                        game.make_public_readable();
                        Ok(warp::reply::with_status(
                            warp::reply::json(&game),
                            StatusCode::OK,
                        ))
                    }
                    None => Ok(reply_error(StatusCode::NOT_FOUND)),
                }
            }
            None => Ok(reply_error(StatusCode::NOT_FOUND)),
        },
        None => Ok(reply_error(StatusCode::UNAUTHORIZED)),
    }
}

pub async fn get_games_count_filter(ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct GetGamesResponse {
        total: usize,
    };

    let total = ctx
        .db()
        .games()
        .total_count()
        .await
        .expect("Reading games count has failed");

    Ok(warp::reply::with_status(
        warp::reply::json(&GetGamesResponse { total }),
        StatusCode::OK,
    ))
}

pub async fn create_game_filter(ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    fn generate_game_token() -> String {
        let mut rng = thread_rng();

        String::from_utf8(
            iter::repeat(())
                .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase())
                .take(TOKEN_CHARS_COUNT)
                .collect(),
        )
        .unwrap()
    }

    let game_token = generate_game_token();
    let player = create_new_player(&game_token, ctx).await;
    let new_game = create_new_game(player.id(), &game_token, ctx).await;

    #[derive(Serialize)]
    struct CreateGameReponse {
        game: Game,
        admin: Player,
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&CreateGameReponse {
            game: new_game,
            admin: player,
        }),
        StatusCode::CREATED,
    ))
}

pub async fn attend_game_filter(
    game_token: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match ctx
        .db()
        .games()
        .get(&game_token)
        .await
        .expect("Reading game has failed")
    {
        Some(mut game) => {
            let player = create_new_player(&game_token, ctx).await;

            game.add_player(player.id());
            match ctx.db().games().persist(&game).await {
                Ok(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&player),
                    StatusCode::OK,
                )),
                Err(_) => Ok(reply_error(StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
        None => Ok(reply_error(StatusCode::NOT_FOUND)),
    }
}

pub async fn leave_game_filter(
    game_token: &str,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match ctx
        .db()
        .games()
        .get(&game_token)
        .await
        .expect("Reading game has failed")
    {
        Some(mut game) => match extract_verified_id(authorization, ctx) {
            Some(player_id) => {
                game.remove_player(&player_id);
                match ctx.db().games().persist(&game).await {
                    Ok(_) => Ok(reply_success(StatusCode::OK)),
                    Err(_) => Ok(reply_error_with_details(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Writing player has failed",
                    )),
                }
            }
            None => Ok(reply_error(StatusCode::UNAUTHORIZED)),
        },
        None => Ok(reply_error(StatusCode::NOT_FOUND)),
    }
}

pub async fn start_game_filter(
    game_token: &str,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match ctx
        .db()
        .games()
        .get(&game_token)
        .await
        .expect("Reading game has failed")
    {
        Some(mut game) => match extract_verified_id(authorization, ctx)
            .filter(|_| game.admin_id().is_some())
            .filter(|id| id == game.admin_id().as_ref().unwrap())
        {
            Some(_) => {
                game.start();
                match ctx.db().games().persist(&game).await {
                    Ok(_) => Ok(reply_error(StatusCode::OK)),
                    Err(_) => Ok(reply_error(StatusCode::INTERNAL_SERVER_ERROR)),
                }
            }
            None => Ok(reply_error(StatusCode::UNAUTHORIZED)),
        },
        None => Ok(reply_error(StatusCode::NOT_FOUND)),
    }
}

async fn create_new_game(admin_id: &str, token: &str, ctx: &AppContext) -> Game {
    let new_game = Game::new(admin_id, token);
    let new_token = new_game.token();
    ctx.db()
        .games()
        .persist(&new_game)
        .await
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);

    new_game
}

async fn create_new_player(game_token: &str, ctx: &AppContext) -> Player {
    let mut player = Player::new(game_token);
    let user_token = generate_jwt_token(&player, &ctx.config().auth_secret);
    player.update_token(&user_token);

    ctx.db()
        .players()
        .persist(&player)
        .await
        .expect("Creating player failed");
    debug!("Created player with token {}", player.id());

    player
}

#[cfg(test)]
mod tests {
    use super::{
        attend_game_filter, create_game_filter, get_game_filter, leave_game_filter,
        start_game_filter,
    };
    use crate::{
        model::{Game, GameState, Party, Player, Role},
        server::{app_context::AppContext, auth::generate_jwt_token},
    };
    use warp::{hyper::StatusCode, Reply};

    const GAME_TOKEN: &str = "ACDEF";

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_not_get_game_unauthorized() {
        let ctx = init_ctx();

        let reply = get_game_filter("invalid", "auth", &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    async fn should_not_get_unknown_game() {
        let ctx = init_ctx();

        let token = generate_jwt_token(&Player::new("game"), &ctx.config().auth_secret);

        let reply = get_game_filter("game", &token, &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_get_game_for_admin() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        ctx.db()
            .players()
            .persist(&admin)
            .await
            .expect("Writing player failed");
        ctx.db()
            .games()
            .persist(&Game::new(admin.id(), GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_get_game_for_player() {
        let ctx = init_ctx();

        let player = Player::new(GAME_TOKEN);
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Writing player failed");
        let mut game = Game::new("admin", GAME_TOKEN);
        game.add_player(player.id());
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");

        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_get_game_and_send_heartbeat() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        let first_time = admin.last_action_time();
        ctx.db()
            .players()
            .persist(&admin)
            .await
            .expect("Writing admin failed");

        ctx.db()
            .games()
            .persist(&Game::new(admin.id(), GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_admin = ctx
            .db()
            .players()
            .get(admin.id())
            .await
            .expect("Reading admin failed");
        assert!(updated_admin.unwrap().last_action_time().gt(&first_time));
    }

    #[tokio::test]
    async fn should_create_new_game() {
        let ctx = init_ctx();

        let reply = create_game_filter(&ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn should_not_attend_unknown_game() {
        let ctx = init_ctx();

        let reply = attend_game_filter("test", &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_attend_game() {
        let ctx = init_ctx();

        ctx.db()
            .games()
            .persist(&Game::new("admin", GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = attend_game_filter(GAME_TOKEN, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_leave_game() {
        let ctx = init_ctx();

        let mut game = Game::new("admin", GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);
        game.add_player(player.id());

        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");
        assert!(game.player_ids().contains(player.id()));

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldnt find game");
        assert!(!updated_game.unwrap().player_ids().contains(player.id()));
    }

    #[tokio::test]
    async fn should_leave_game_and_select_new_admin() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        let mut game = Game::new(admin.id(), GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        game.add_player(player.id());

        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");
        assert_eq!(game.admin_id().as_ref().unwrap(), admin.id());

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldnt find game")
            .unwrap();
        assert!(updated_game.player_ids().is_empty());
        assert_eq!(updated_game.admin_id().as_ref().unwrap(), player.id());
    }

    #[tokio::test]
    async fn should_start_game() {
        let ctx = init_ctx();
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        ctx.db()
            .games()
            .persist(&Game::new(player.id(), GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = start_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldnt find game")
            .unwrap();
        assert_eq!(updated_game.state(), &GameState::Started);
        assert_eq!(
            updated_game.assigned_roles().get(player.id()).unwrap(),
            &Role::new("Good", Party::Good)
        );
        assert_eq!(updated_game.assigned_roles().len(), 1);
    }

    #[tokio::test]
    async fn should_not_start_game() {
        let ctx = init_ctx();
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        ctx.db()
            .games()
            .persist(&Game::new("admin", GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = start_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::UNAUTHORIZED
        );

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldnt find game");
        assert_eq!(updated_game.unwrap().state(), &GameState::Initialized);
    }

    #[tokio::test]
    async fn should_not_start_unknown_game() {
        let ctx = init_ctx();
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = start_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }
}
