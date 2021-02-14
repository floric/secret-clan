use crate::{
    model::{Game, GameState, Player, TaskDefinition},
    server::{
        app_context::AppContext,
        auth::{extract_verified_id, generate_jwt_token},
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

#[derive(Serialize)]
struct AttendGameReponse {
    game: String,
    token: String,
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

    Ok(warp::reply::with_status(
        warp::reply::json(&AttendGameReponse {
            game: String::from(new_game.token()),
            token: String::from(player.user_token()),
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
        .filter(|game| game.state() != &GameState::Started)
    {
        Some(mut game) => {
            let player = create_new_player(&game_token, ctx).await;

            game.add_player(player.id(), player.position());

            match ctx.db().games().persist(&game).await {
                Ok(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&AttendGameReponse {
                        game: String::from(game.token()),
                        token: String::from(player.user_token()),
                    }),
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
    player.assign_task(TaskDefinition::Settings {});

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
    use super::{attend_game_filter, create_game_filter, leave_game_filter};
    use crate::{
        model::{Game, GameState, Player},
        server::{app_context::AppContext, auth::generate_jwt_token},
    };
    use warp::{hyper::StatusCode, Reply};

    const GAME_TOKEN: &str = "ACDEF";

    #[tokio::test]
    async fn should_create_new_game() {
        let ctx = AppContext::init();

        let reply = create_game_filter(&ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn should_not_attend_unknown_game() {
        let ctx = AppContext::init();

        let reply = attend_game_filter("test", &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_not_attend_started_game() {
        let ctx = AppContext::init();

        let mut game = Game::new("admin", GAME_TOKEN);
        game.start();
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");

        let reply = attend_game_filter(GAME_TOKEN, &ctx).await;
        assert_eq!(
            reply.unwrap().into_response().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_attend_game() {
        let ctx = AppContext::init();

        ctx.db()
            .games()
            .persist(&Game::new("admin", GAME_TOKEN))
            .await
            .expect("Writing game failed");

        let reply = attend_game_filter(GAME_TOKEN, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldn't find game");
        assert_eq!(updated_game.unwrap().state(), &GameState::Initialized);
    }

    #[tokio::test]
    async fn should_leave_game() {
        let ctx = AppContext::init();

        let mut game = Game::new("admin", GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);
        game.add_player(player.id(), 1);

        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");
        assert!(game.all_player_ids().contains(&String::from(player.id())));

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldn't find game")
            .unwrap();
        assert!(!updated_game
            .all_player_ids()
            .contains(&String::from(player.id())));
        assert_eq!(updated_game.state(), &GameState::Initialized);
    }

    #[tokio::test]
    async fn should_abandone_game() {
        let ctx = AppContext::init();

        let player = Player::new(GAME_TOKEN);
        let game = Game::new(player.id(), GAME_TOKEN);
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldn't find game")
            .unwrap();
        assert!(!updated_game
            .all_player_ids()
            .contains(&String::from(player.id())));
        assert_eq!(updated_game.state(), &GameState::Abandoned);
    }

    #[tokio::test]
    async fn should_leave_game_and_select_new_admin() {
        let ctx = AppContext::init();

        let admin = Player::new(GAME_TOKEN);
        let mut game = Game::new(admin.id(), GAME_TOKEN);
        let player = Player::new(GAME_TOKEN);
        let token = generate_jwt_token(&admin, &ctx.config().auth_secret);
        game.add_player(player.id(), 1);

        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");
        assert_eq!(game.admin_id().as_ref().unwrap(), admin.id());
        assert_eq!(game.all_player_ids().len(), 2);

        let reply = leave_game_filter(GAME_TOKEN, &token, &ctx).await;
        assert_eq!(reply.unwrap().into_response().status(), StatusCode::OK);

        let updated_game = ctx
            .db()
            .games()
            .get(GAME_TOKEN)
            .await
            .expect("Couldn't find game")
            .unwrap();
        assert_eq!(updated_game.all_player_ids().len(), 1);
        assert_eq!(updated_game.admin_id().as_ref().unwrap(), player.id());
        assert_eq!(updated_game.state(), &GameState::Initialized);
    }
}
