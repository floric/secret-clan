use log::{debug, info};
use sled::{Db, IVec};
use std::{clone::Clone, convert::TryFrom, marker::PhantomData};

use crate::model::{game::Game, player::Player};

pub struct Repository<T> {
    path: String,
    tree: Db,
    phantom: PhantomData<T>,
}

impl<T: Persist + Into<IVec> + TryFrom<IVec> + Clone> Repository<T> {
    pub fn init(path: &str) -> Repository<T> {
        let tree = sled::open(format!(
            "{}/.sled/{}",
            dirs::home_dir()
                .expect("No user dir known")
                .to_str()
                .unwrap(),
            &path
        ))
        .expect("opening database has failed");

        let repo = Repository {
            tree,
            path: String::from(path),
            phantom: PhantomData,
        };

        repo.purge_data()
            .expect("Cleanup of existing database has failed");

        debug!("Database on \"{}\" ready", repo.path);

        repo
    }

    pub fn persist(&self, elem: T) -> Result<bool, String> {
        self.tree
            .insert(elem.id(), elem.clone())
            .expect("Persisting item failed");
        self.flush().map_err(|e| e.to_string()).map(|_| true)
    }

    pub fn find_by_id(&self, id: &str) -> Option<T> {
        let success = self.tree.get(id);
        match success {
            Ok(res) => res.and_then(|g| T::try_from(g).ok()),
            Err(_) => None,
        }
    }

    pub fn total_count(&self) -> usize {
        self.tree.len()
    }

    fn flush(&self) -> Result<bool, sled::Error> {
        self.tree.flush().map(|_| true)
    }

    fn purge_data(&self) -> Result<usize, sled::Error> {
        info!("Try to purge database \"{}\"", self.path);
        self.tree.clear().and_then(|()| self.tree.flush())
    }
}

pub trait Persist {
    fn id(&self) -> &str;
}

pub struct Repositories {
    games: Repository<Game>,
    players: Repository<Player>,
}

impl Repositories {
    pub fn init() -> Repositories {
        Repositories {
            games: Repository::init("games"),
            players: Repository::init("players"),
        }
    }

    pub fn games(&self) -> &Repository<Game> {
        &self.games
    }

    pub fn players(&self) -> &Repository<Player> {
        &self.players
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
            .persist(Game::new())
            .expect("Game persist failed");
    }

    #[test]
    fn should_find_game() {
        let ctx = init_ctx();
        let game = Game::new();
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
            .persist(Game::new())
            .expect("Game persist failed");

        assert_eq!(ctx.repos().games().total_count(), 1);

        ctx.repos()
            .games()
            .purge_data()
            .expect("Cleanup has failed");

        assert_eq!(ctx.repos().games().total_count(), 0);
    }
}
