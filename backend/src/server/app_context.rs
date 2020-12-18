use super::logger::init_logger;
use crate::{
    config::AppConfig,
    model::{game::Game, player::Player},
    persistence::{AsyncRepository, Command, Repository},
};
use envconfig::Envconfig;
use tokio::sync::mpsc;

pub struct Repositories {
    games_async: mpsc::Sender<Command<Game>>,
    players: Repository<Player>,
}

impl Repositories {
    pub fn init(games_async: mpsc::Sender<Command<Game>>) -> Repositories {
        Repositories {
            games_async,
            players: Repository::init("players"),
        }
    }

    pub fn games_async(&self) -> mpsc::Sender<Command<Game>> {
        self.games_async.clone()
    }

    pub fn players(&self) -> &Repository<Player> {
        &self.players
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
        let games_async = games_repo.sender();
        tokio::spawn(async move {
            games_repo.start_listening().await;
        });

        let repos = Repositories::init(games_async);

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
