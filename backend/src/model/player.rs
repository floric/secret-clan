use crate::db::Persist;
use chrono::{DateTime, Utc};
use names::Generator;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::hash::Hash;

fn generate_random_name() -> String {
    Generator::default().next().unwrap()
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
pub struct Player {
    id: String,
    name: String,
    game_token: String,
    #[derivative(Debug = "ignore")]
    user_token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
}

impl Player {
    pub fn new(game_token: &str) -> Self {
        Player {
            id: nanoid!(),
            name: generate_random_name(),
            game_token: String::from(game_token),
            user_token: String::from(""),
            creation_time: Utc::now(),
            last_action_time: Utc::now(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        if !name.is_empty() {
            self.name = String::from(name);
        }
    }

    pub fn game_token(&self) -> &str {
        &self.game_token
    }

    pub fn update_token(&mut self, new_token: &str) {
        self.user_token = String::from(new_token);
    }

    pub fn heartbeat(&mut self) {
        self.last_action_time = Utc::now();
    }

    pub fn last_action_time(&self) -> DateTime<Utc> {
        self.last_action_time
    }
}

impl Persist for Player {
    fn id(&self) -> &str {
        self.id()
    }
}

impl Into<IVec> for Player {
    fn into(self) -> IVec {
        IVec::from(bincode::serialize(&self).unwrap())
    }
}

impl From<IVec> for Player {
    fn from(bytes: IVec) -> Player {
        let vec: Vec<u8> = bytes.to_vec();
        bincode::deserialize(&vec).unwrap()
    }
}
