use super::{logger::init_logger, ws::WsClient};
use crate::{
    config::AppConfig,
    db::{ChangeListener, Client, Database},
    model::{Game, Player},
};
use envconfig::Envconfig;
use tokio::sync::mpsc;

pub struct DbClients {
    games: Client<Game>,
    players: Client<Player>,
}

impl DbClients {
    pub fn init_with_changes() -> (DbClients, ChangeListener) {
        let (mut games_repo, games_sender) = Database::init("games");
        let (mut players_repo, players_sender) = Database::init("players");

        let (players, player_changes): (Client<Player>, mpsc::Receiver<Player>) =
            Client::new_with_change_handler(players_sender);
        let (games, game_changes): (Client<Game>, mpsc::Receiver<Game>) =
            Client::new_with_change_handler(games_sender);

        tokio::task::spawn(async move {
            tokio::join!(players_repo.start_listening(), games_repo.start_listening(),);
        });

        (
            DbClients { games, players },
            ChangeListener::new(player_changes, game_changes),
        )
    }

    pub fn init() -> DbClients {
        let (mut games_repo, games_sender) = Database::init("games");
        let (mut players_repo, players_sender) = Database::init("players");

        tokio::task::spawn(async move {
            tokio::join!(players_repo.start_listening(), games_repo.start_listening(),);
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

/// The AppContext is the central place for crosscutting topics like Database acess or reading configuration values.
/// Each request filter gets a reference to the context to read from it or query requests by sending messages.
/// These messages will be sent to separates threads for mutations so we can sure no mutations occure directly in this shared, readonly state object.
pub struct AppContext {
    pub db: DbClients,
    pub ws: WsClient,
    pub config: AppConfig,
}

impl AppContext {
    pub fn init() -> AppContext {
        let (config, ws) = AppContext::init_config_and_ws();
        let db = DbClients::init();

        AppContext { config, ws, db }
    }

    pub fn init_with_changes() -> (AppContext, ChangeListener) {
        let (config, ws) = AppContext::init_config_and_ws();
        let (db, changes) = DbClients::init_with_changes();

        (AppContext { config, ws, db }, changes)
    }

    fn init_config_and_ws() -> (AppConfig, WsClient) {
        let config = AppConfig::init_from_env().expect("Loading server config failed");
        let ws = WsClient::default();

        init_logger(&config);

        (config, ws)
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

    pub fn ws(&self) -> &WsClient {
        &self.ws
    }
}
