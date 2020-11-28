use crate::{
    model::game::Game,
    model::player::Player,
    server::auth::generate_jwt_token,
    server::CreatePlayerInput,
    server::{app_context::AppContext, errors::reply_with_error},
};
use log::debug;
use serde::{Deserialize, Serialize};
use warp::hyper::StatusCode;
use warp::Filter;

const GAMES_PATH: &str = "games";

pub fn get_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::path!(String))
        .map(move |token: String| {
            let new_game = get_game_by_token(&ctx, &token);
            if new_game.is_none() {
                return reply_with_error(StatusCode::NOT_FOUND);
            }

            warp::reply::with_status(warp::reply::json(&new_game.unwrap()), StatusCode::OK)
        })
}

pub fn get_games_count(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::get())
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
}

pub fn create_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH).and(warp::put()).map(move || {
        let new_game = create_new_game(&ctx);
        warp::reply::with_status(warp::reply::json(&new_game), StatusCode::CREATED)
    })
}

pub fn attend_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH)
        .and(warp::post())
        .and(warp::path!(String / "attend"))
        .and(warp::body::json())
        .map(move |game_token: String, input: CreatePlayerInput| {
            let new_game = get_game_by_token(&ctx, &game_token);
            if new_game.is_none() {
                return reply_with_error(StatusCode::NOT_FOUND);
            }

            let mut player = Player::new(&input.name, &game_token);
            let user_token = generate_jwt_token(&player, &ctx.config().auth_secret);
            player.update_token(&user_token);

            ctx.repos()
                .players()
                .persist(player.clone())
                .expect("Creating player failed");

            warp::reply::with_status(warp::reply::json(&player), StatusCode::OK)
        })
}

fn create_new_game(ctx: &AppContext) -> Game {
    let new_game = Game::new();
    let new_token = new_game.token();
    ctx.repos()
        .games()
        .persist(new_game.clone())
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);
    new_game
}

fn get_game_by_token(ctx: &AppContext, token: &str) -> Option<Game> {
    ctx.repos().games().find_by_id(&token.to_uppercase())
}
