mod config;
mod logic;
mod model;
mod persistence;
mod server;

use model::game::Game;
use persistence::Persist;
use server::run_server;

extern crate chrono;
extern crate envconfig;
extern crate log;

#[tokio::main]
async fn main() {
    // clean old data
    Game::purge_data(None::<Game>);

    run_server().await;
}
