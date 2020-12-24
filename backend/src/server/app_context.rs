use super::logger::init_logger;
use crate::{
    config::AppConfig,
    db::{Client, Database},
    model::{Game, Player},
};
use envconfig::Envconfig;

pub struct DbClients {
    games: Client<Game>,
    players: Client<Player>,
}

impl DbClients {
    pub fn init() -> DbClients {
        let mut games_repo = Database::init("games");
        let games_sender = games_repo.sender();
        tokio::task::spawn(async move {
            games_repo.start_listening().await;
        });
        let mut players_repo = Database::init("players");
        let players_sender = players_repo.sender();
        tokio::task::spawn(async move {
            players_repo.start_listening().await;
        });

        DbClients {
            games: Client::new(games_sender),
            players: Client::new(players_sender),
        }
    }

    pub fn games(&self) -> &Client<Game> {
        &self.games
    }

    pub fn players(&self) -> &Client<Player> {
        &self.players
    }
}

pub struct AppContext {
    db: DbClients,
    config: AppConfig,
}

impl AppContext {
    pub fn init() -> AppContext {
        let config = AppConfig::init_from_env().expect("Loading server config failed");

        init_logger(&config);

        let db = DbClients::init();

        AppContext { db, config }
    }

    pub fn db(&self) -> &DbClients {
        &self.db
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn is_dev(&self) -> bool {
        cfg!(debug_assertions)
    }
}
