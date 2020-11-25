use log::debug;

use crate::model::game::Game;
use crate::persistence::Persist;

pub fn create_new_game() -> Game {
    let new_game = Game::new();
    new_game.persist().expect("Creating game failed");
    debug!("Created game with token {}", new_game.token());
    new_game
}

pub fn get_game_by_token(token: &str) -> Option<Game> {
    Game::find_by_id(None::<Game>, &token.to_uppercase())
}
