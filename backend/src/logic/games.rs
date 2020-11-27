use log::debug;

use crate::{model::game::Game, server::app_context::AppContext};

pub fn create_new_game(ctx: &AppContext) -> Game {
    let new_game = Game::new();
    let new_token = new_game.token().clone();
    ctx.repos()
        .games()
        .persist(new_game.clone())
        .expect("Creating game failed");
    debug!("Created game with token {}", new_token);
    new_game
}

pub fn get_game_by_token(ctx: &AppContext, token: &str) -> Option<Game> {
    ctx.repos().games().find_by_id(&token.to_uppercase())
}
