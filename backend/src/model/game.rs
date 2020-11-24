use crate::persistence::Persist;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{hash::Hash, iter};

const TOKEN_CHARS_COUNT: usize = 5;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    token: String,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let token: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase())
            .take(TOKEN_CHARS_COUNT)
            .collect();

        Game { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Persist<Game> for Game {
    fn id(&self) -> &str {
        self.token()
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

impl From<IVec> for Game {
    fn from(bytes: IVec) -> Game {
        let vec: Vec<u8> = bytes.to_vec();
        bincode::deserialize(&vec).unwrap()
    }
}
