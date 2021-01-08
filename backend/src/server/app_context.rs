use super::logger::init_logger;
use crate::{
    config::AppConfig,
    db::{Client, Database},
    model::{Game, Player, Voting},
};
use envconfig::Envconfig;

pub struct DbClients {
    games: Client<Game>,
    players: Client<Player>,
    votings: Client<Voting>,
}

impl DbClients {
    pub fn init() -> DbClients {
        let (mut games_repo, games_sender) = Database::init("games");
        let (mut players_repo, players_sender) = Database::init("players");
        let (mut votings_repo, votings_sender) = Database::init("votings");

        tokio::task::spawn(async move {
            tokio::join!(
                players_repo.start_listening(),
                games_repo.start_listening(),
                votings_repo.start_listening()
            );
        });

        DbClients {
            games: Client::new(games_sender),
            players: Client::new(players_sender),
            votings: Client::new(votings_sender),
        }
    }

    pub fn games(&self) -> &Client<Game> {
        &self.games
    }

    pub fn players(&self) -> &Client<Player> {
        &self.players
    }

    pub fn votings(&self) -> &Client<Voting> {
        &self.votings
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
