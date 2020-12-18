use crate::{
    model::{game::Game, player::Player},
    persistence::AsyncRepository,
    server::{
        app_context::AppContext, auth::generate_jwt_token, auth::verify_jwt_token,
        errors::reply_with_error,
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

// GET /api/games/:token
pub async fn get_game_filter(
    game_token: &str,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match verify_jwt_token(authorization, &ctx.config().auth_secret)
        .ok()
        .and_then(|token| token.claims().get("sub").map(String::from))
    {
        Some(token) => match get_game_by_token(ctx, game_token).await {
            Some(game) => match get_player_by_id(ctx, &token).await.filter(|player| {
                game.player_ids().contains(player.id())
                    || game.admin_id().is_some() && game.admin_id().as_ref().unwrap() == player.id()
            }) {
                Some(mut player) => {
                    player.heartbeat();
                    persist_player(ctx, &player)
                        .await
                        .expect("Persisting heartbeat has failed");
                    Ok(warp::reply::with_status(
                        warp::reply::json(&game),
                        StatusCode::OK,
                    ))
                }
                None => Ok(reply_with_error(StatusCode::NOT_FOUND)),
            },
            None => Ok(reply_with_error(StatusCode::NOT_FOUND)),
        },
        None => Ok(reply_with_error(StatusCode::UNAUTHORIZED)),
    }
}

// GET /api/games/
pub async fn get_games_count_filter(ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct GetGamesResponse {
        total: usize,
    };

    let s = ctx.repos().games();
    let total = AsyncRepository::total_count_async(&s).await;

    Ok(warp::reply::with_status(
        warp::reply::json(&GetGamesResponse { total }),
        StatusCode::OK,
    ))
}

// PUT /api/games/
pub async fn create_game_filter(ctx: &AppContext) -> Result<impl warp::Reply, Infallible> {
    fn generate_game_token() -> String {
        let mut rng = thread_rng();

        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase())
            .take(TOKEN_CHARS_COUNT)
            .collect()
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

// POST /api/games/attend
pub async fn attend_game_filter(
    game_token: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match get_game_by_token(ctx, &game_token).await {
        Some(mut game) => {
            let player = create_new_player(&game_token, ctx).await;

            game.add_player(player.id());
            match persist_game(ctx, &game).await {
                Ok(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&player),
                    StatusCode::OK,
                )),
                Err(_) => Ok(reply_with_error(StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
        None => Ok(reply_with_error(StatusCode::NOT_FOUND)),
    }
}

// POST /api/games/leave
pub async fn leave_game_filter(
    game_token: &str,
    authorization: &str,
    ctx: &AppContext,
) -> Result<impl warp::Reply, Infallible> {
    match get_game_by_token(ctx, &game_token).await {
        Some(mut game) => match verify_jwt_token(&authorization, &ctx.config().auth_secret)
            .map_or(None, |token| token.claims().get("sub").map(String::from))
        {
            Some(player_id) => {
                game.remove_player(&player_id);
                match game.admin_id() {
                    Some(_) => match persist_game(ctx, &game).await {
                        Ok(_) => Ok(reply_with_error(StatusCode::OK)),
                        Err(_) => Ok(reply_with_error(StatusCode::INTERNAL_SERVER_ERROR)),
                    },
                    None => match remove_game(ctx, game.token()).await {
                        Ok(_) => Ok(reply_with_error(StatusCode::OK)),
                        Err(_) => Ok(reply_with_error(StatusCode::INTERNAL_SERVER_ERROR)),
                    },
                }
            }
            None => Ok(reply_with_error(StatusCode::UNAUTHORIZED)),
        },
        None => Ok(reply_with_error(StatusCode::NOT_FOUND)),
    }
}

async fn create_new_game(admin_id: &str, token: &str, ctx: &AppContext) -> Game {
    let new_game = Game::new(admin_id, token);
    let new_token = new_game.token();
    persist_game(ctx, &new_game)
        .await
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);

    new_game
}

async fn create_new_player(game_token: &str, ctx: &AppContext) -> Player {
    let mut player = Player::new(game_token);
    let user_token = generate_jwt_token(&player, &ctx.config().auth_secret);
    player.update_token(&user_token);

    persist_player(ctx, &player)
        .await
        .expect("Creating player failed");
    debug!("Created player with token {}", player.id());

    player
}

pub async fn get_game_by_token(ctx: &AppContext, token: &str) -> Option<Game> {
    let s = ctx.repos().games();
    AsyncRepository::find_by_id_async(&token.to_uppercase(), &s).await
}

pub async fn get_player_by_id(ctx: &AppContext, id: &str) -> Option<Player> {
    let s = ctx.repos().players();
    AsyncRepository::find_by_id_async(&id, &s).await
}

pub async fn persist_game(ctx: &AppContext, game: &Game) -> Result<bool, String> {
    let s = ctx.repos().games();
    AsyncRepository::persist_async(game, &s).await
}

pub async fn persist_player(ctx: &AppContext, player: &Player) -> Result<bool, String> {
    let s = ctx.repos().players();
    AsyncRepository::persist_async(player, &s).await
}

pub async fn remove_game(ctx: &AppContext, game_id: &str) -> Result<bool, String> {
    let s = ctx.repos().games();
    AsyncRepository::remove_async(game_id, &s).await
}

#[cfg(test)]
mod tests {
    use super::{
        attend_game_filter, create_game_filter, get_game_by_token, get_game_filter,
        get_player_by_id, leave_game_filter, persist_game, persist_player,
    };
    use crate::{
        model::{game::Game, player::Player},
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
        persist_player(&ctx, &admin)
            .await
            .expect("Writing player failed");
        persist_game(&ctx, &Game::new(admin.id(), GAME_TOKEN))
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
        persist_player(&ctx, &player)
            .await
            .expect("Writing player failed");
        let mut game = Game::new("admin", GAME_TOKEN);
        game.add_player(player.id());
        persist_game(&ctx, &game)
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
        persist_player(&ctx, &admin)
            .await
            .expect("Writing admin failed");

        persist_game(&ctx, &Game::new(admin.id(), GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = get_game_filter(GAME_TOKEN, &token, &ctx).await;

        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_admin = get_player_by_id(&ctx, admin.id())
            .await
            .expect("Reading admin failed");
        assert!(updated_admin.last_action_time().gt(&first_time));
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

        persist_game(&ctx, &Game::new("admin", GAME_TOKEN))
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

        persist_game(&ctx, &game)
            .await
            .expect("Writing game failed");
        assert!(game.player_ids().contains(player.id()));

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = get_game_by_token(&ctx, GAME_TOKEN)
            .await
            .expect("Couldnt find game");
        assert!(!updated_game.player_ids().contains(player.id()));
    }

    #[tokio::test]
    async fn should_leave_game_and_select_new_admin() {
        let ctx = init_ctx();

        let admin = Player::new(GAME_TOKEN);
        let mut game = Game::new(admin.id(), GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        game.add_player(player.id());

        persist_game(&ctx, &game)
            .await
            .expect("Writing game failed");

        assert_eq!(game.admin_id().as_ref().unwrap(), admin.id());

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;

        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = get_game_by_token(&ctx, GAME_TOKEN)
            .await
            .expect("Couldnt find game");
        assert!(updated_game.player_ids().is_empty());
        assert_eq!(updated_game.admin_id().as_ref().unwrap(), player.id());
    }
}
