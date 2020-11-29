use crate::{
    model::game::Game,
    model::player::Player,
    server::auth::generate_jwt_token,
    server::{app_context::AppContext, errors::reply_with_error},
};
use log::debug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::iter;
use warp::hyper::StatusCode;
use warp::Filter;

const GAMES_PATH: &str = "games";

// this value determines the findability of a game and is a tradeoff between security and user friendliness
// 5 tokens mean a chance of finding a random game of 1:60466176 ()
const TOKEN_CHARS_COUNT: usize = 5;

// GET /api/games/:token
pub fn get_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(GAMES_PATH).and(warp::path!(String)).map(
        move |token: String| match get_game_by_token(&ctx, &token) {
            Some(game) => warp::reply::with_status(warp::reply::json(&game), StatusCode::OK),
            None => reply_with_error(StatusCode::NOT_FOUND),
        },
    )
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
    fn generate_game_token() -> String {
        let mut rng = thread_rng();

        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase())
            .take(TOKEN_CHARS_COUNT)
            .collect()
    }

    warp::path(GAMES_PATH).and(warp::put()).map(move || {
        let game_token = generate_game_token();
        // TODO insert name of Admin
        let player = create_new_player("Admin", &game_token, ctx);
        let new_game = create_new_game(player.id(), &game_token, &ctx);

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
    })
}

// POST /api/games/attend
pub fn attend_game(
    ctx: &'static AppContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    #[derive(Deserialize)]
    struct CreatePlayerInput {
        name: String,
    }

    warp::path(GAMES_PATH)
        .and(warp::post())
        .and(warp::path!(String / "attend"))
        .and(warp::body::json())
        .map(move |game_token: String, input: CreatePlayerInput| {
            match get_game_by_token(&ctx, &game_token) {
                Some(mut new_game) => {
                    let player = create_new_player(&input.name, &game_token, ctx);

                    new_game.add_player(player.id());
                    ctx.repos()
                        .games()
                        .persist(new_game)
                        .expect("Adding player failed");

                    warp::reply::with_status(warp::reply::json(&player), StatusCode::OK)
                }
                None => reply_with_error(StatusCode::NOT_FOUND),
            }
        })
}

fn create_new_game(admin_id: &str, token: &str, ctx: &AppContext) -> Game {
    let new_game = Game::new(admin_id, token);
    let new_token = new_game.token();
    ctx.repos()
        .games()
        .persist(new_game.clone())
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);

    new_game
}

fn create_new_player(name: &str, game_token: &str, ctx: &AppContext) -> Player {
    let mut player = Player::new(&name, game_token);
    let user_token = generate_jwt_token(&player, &ctx.config().auth_secret);
    player.update_token(&user_token);

    ctx.repos()
        .players()
        .persist(player.clone())
        .expect("Creating player failed");
    debug!("Created player with token {}", player.id());

    player
}

fn get_game_by_token(ctx: &AppContext, token: &str) -> Option<Game> {
    ctx.repos().games().find_by_id(&token.to_uppercase())
}
