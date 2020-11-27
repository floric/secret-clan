use log::warn;
use sled::{Db, IVec};
use std::{clone::Clone, convert::TryFrom, marker::PhantomData};

use crate::model::game::Game;

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

        repo.purge_data();

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

    fn flush(&self) -> Result<bool, sled::Error> {
        self.tree.flush().map(|_| true)
    }

    fn purge_data(&self) {
        let res = self.tree.drop_tree(&self.path);
        if res.is_err() {
            warn!("Cleaning database has failed");
        }
    }
}

pub trait Persist {
    fn id(&self) -> &str;
}

pub struct Repositories {
    games: Repository<Game>,
}

impl Repositories {
    pub fn init() -> Repositories {
        Repositories {
            games: Repository::init("games"),
        }
    }

    pub fn games(&self) -> &Repository<Game> {
        &self.games
    }
}
