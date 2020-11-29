use crate::config::AppConfig;
use crate::persistence::Repositories;
use envconfig::Envconfig;

use super::logger::init_logger;

pub struct AppContext {
    repos: Repositories,
    config: AppConfig,
}

impl AppContext {
    pub fn init() -> AppContext {
        let config = AppConfig::init_from_env().expect("Loading server config failed");
        init_logger(&config);
        let repos = Repositories::init();

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
