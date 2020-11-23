use crate::persistence::Persist;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    id: String,
    token: String,
}

impl Game {
    pub fn new(token: &str) -> Game {
        Game {
            id: nanoid!(),
            token: String::from(token),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Persist<Game> for Game {
    fn id(&self) -> &str {
        self.id()
    }

    fn persistence_path(&self) -> String {
        String::from("/games")
    }
}

impl Into<IVec> for Game {
    fn into(self) -> IVec {
        IVec::from(bincode::serialize(&self).unwrap())
    }
}
