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

        AppContext {
            repos: Repositories::init(),
            config,
        }
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

#[cfg(test)]
mod tests {
    use crate::model::game::Game;
    use crate::persistence::Persist;

    use super::*;
    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[test]
    fn should_persist_game() {
        // when
        let ctx = init_ctx();

        // when, then
        ctx.repos
            .games()
            .persist(Game::new())
            .expect("Game persist failed");
    }

    #[test]
    fn should_find_game() {
        // given
        let ctx = init_ctx();
        let game = Game::new();
        let game_id = String::from(game.id());
        ctx.repos
            .games()
            .persist(game)
            .expect("Game persist failed");

        // when
        let res = ctx.repos.games().find_by_id(&game_id);

        // then
        assert!(res.is_some());
    }

    #[test]
    fn should_not_find_game() {
        let ctx = init_ctx();
        let res = ctx.repos.games().find_by_id("unknown");
        assert!(res.is_none());
    }
}
