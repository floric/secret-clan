use log::{debug, info};
use sled::Db;
use std::marker::PhantomData;

use super::Persist;

pub struct Repository<T> {
    path: String,
    db: Db,
    phantom: PhantomData<T>,
}

impl<T: Persist> Repository<T> {
    pub fn init(path: &str) -> Repository<T> {
        let full_path = format!("/mnt/efs/.sled/{}", &path);
        info!("Try to open {}", &full_path);
        let db = sled::open(&full_path).expect("Initializing database has failed");

        let repo = Repository {
            db,
            path: String::from(path),
            phantom: PhantomData,
        };

        repo.purge_data()
            .expect("Cleanup of existing database has failed");

        debug!("Database on \"{}\" ready", repo.path);

        repo
    }

    pub fn persist(&self, elem: T) -> Result<bool, String> {
        self.db
            .insert(elem.id(), elem.clone())
            .expect("Persisting item failed");
        self.flush().map_err(|e| e.to_string()).map(|_| true)
    }

    pub fn find_by_id(&self, id: &str) -> Option<T> {
        let success = self.db.get(id);
        match success {
            Ok(res) => res.and_then(|g| T::try_from(g).ok()),
            Err(_) => None,
        }
    }

    pub fn total_count(&self) -> usize {
        self.db.len()
    }

    fn flush(&self) -> Result<bool, sled::Error> {
        self.db.flush().map(|_| true)
    }

    fn purge_data(&self) -> Result<usize, sled::Error> {
        let res = self.db.clear().and_then(|()| self.db.flush());
        info!("Purged database \"{}\"", self.path);

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::Persist;
    use crate::{model::game::Game, server::app_context::AppContext};

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[test]
    fn should_persist_game() {
        let ctx = init_ctx();

        ctx.repos()
            .games()
            .persist(Game::new("admin", "token"))
            .expect("Game persist failed");
    }

    #[test]
    fn should_find_game() {
        let ctx = init_ctx();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());
        ctx.repos()
            .games()
            .persist(game)
            .expect("Game persist failed");

        let res = ctx.repos().games().find_by_id(&game_id);

        assert!(res.is_some());
    }

    #[test]
    fn should_not_find_game() {
        let ctx = init_ctx();
        let res = ctx.repos().games().find_by_id("unknown");

        assert!(res.is_none());
    }

    #[test]
    fn should_purge_games() {
        let ctx = init_ctx();
        ctx.repos()
            .games()
            .persist(Game::new("admin", "token"))
            .expect("Game persist failed");

        assert_eq!(ctx.repos().games().total_count(), 1);

        ctx.repos()
            .games()
            .purge_data()
            .expect("Cleanup has failed");

        assert_eq!(ctx.repos().games().total_count(), 0);
    }
}
