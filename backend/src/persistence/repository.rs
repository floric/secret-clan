use log::{debug, info};
use nanoid::nanoid;
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
        let db = sled::open(if cfg!(test) {
            format!(".sled/{}/{}", nanoid!(), &path)
        } else {
            format!(".sled/{}", &path)
        })
        .expect("opening database has failed");

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

    pub fn persist(&self, elem: &T) -> Result<bool, String> {
        self.db
            .insert(elem.id(), elem.clone())
            .expect("Persisting item failed");
        self.flush().map_err(|e| e.to_string()).map(|_| true)
    }

    pub fn remove(&self, elem: &T) -> Result<bool, String> {
        self.db.remove(elem.id()).expect("Removing item failed");
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
    use super::Repository;
    use crate::model::game::Game;
    use crate::persistence::Persist;

    fn init_ctx() -> Repository<Game> {
        Repository::init("games")
    }

    #[test]
    fn should_persist_game() {
        let ctx = init_ctx();

        ctx.persist(&Game::new("admin", "token"))
            .expect("Game persist failed");
    }

    #[test]
    fn should_find_game() {
        let ctx = init_ctx();
        let game = Game::new("admin", "token");
        let game_id = String::from(game.id());
        ctx.persist(&game).expect("Game persist failed");

        let res = ctx.find_by_id(&game_id);

        assert!(res.is_some());
    }

    #[test]
    fn should_remove_game() {
        let ctx = init_ctx();
        let game = Game::new("admin", "token");
        ctx.persist(&game).expect("Game persist failed");

        let persisted_game = ctx.find_by_id(&game.id());
        assert!(persisted_game.is_some());

        let res = ctx.remove(&game).expect("Removing game failed");
        assert!(res);

        let removed_game = ctx.find_by_id(&game.id());
        assert!(removed_game.is_none());
    }

    #[test]
    fn should_not_find_game() {
        let ctx = init_ctx();
        let res = ctx.find_by_id("unknown");

        assert!(res.is_none());
    }

    #[test]
    fn should_purge_games() {
        let ctx = init_ctx();
        ctx.persist(&Game::new("admin", "token"))
            .expect("Game persist failed");

        assert_eq!(ctx.total_count(), 1);

        ctx.purge_data().expect("Cleanup has failed");

        assert_eq!(ctx.total_count(), 0);
    }
}
