use super::logger::init_logger;
use crate::{
    config::AppConfig,
    model::{game::Game, player::Player},
    persistence::{AsyncRepository, Command},
};
use envconfig::Envconfig;
use tokio::sync::mpsc;

pub struct Repositories {
    games: mpsc::Sender<Command<Game>>,
    players: mpsc::Sender<Command<Player>>,
}

impl Repositories {
    pub fn init(
        games: mpsc::Sender<Command<Game>>,
        players: mpsc::Sender<Command<Player>>,
    ) -> Repositories {
        Repositories { games, players }
    }

    pub fn games(&self) -> mpsc::Sender<Command<Game>> {
        self.games.clone()
    }

    pub fn players(&self) -> mpsc::Sender<Command<Player>> {
        self.players.clone()
    }
}

pub struct AppContext {
    repos: Repositories,
    config: AppConfig,
}

impl AppContext {
    pub fn init() -> AppContext {
        let config = AppConfig::init_from_env().expect("Loading server config failed");
        init_logger(&config);

        let mut games_repo = AsyncRepository::init("games");
        let games = games_repo.sender();
        tokio::spawn(async move {
            games_repo.start_listening().await;
        });
        let mut players_repo = AsyncRepository::init("players");
        let players = players_repo.sender();
        tokio::spawn(async move {
            players_repo.start_listening().await;
        });

        let repos = Repositories::init(games, players);

        AppContext { repos, config }
    }

    pub fn repos(&self) -> &Repositories {
        &self.repos
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn is_dev(&self) -> bool {
        cfg!(debug_assertions)
    }
}
