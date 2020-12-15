use crate::persistence::Persist;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{collections::BTreeSet, convert::TryFrom, hash::Hash};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
    admin_id: String,
    player_ids: BTreeSet<String>,
}

impl Game {
    pub fn new(admin_id: &str, token: &str) -> Game {
        Game {
            token: String::from(token).to_uppercase(),
            creation_time: Utc::now(),
            last_action_time: Utc::now(),
            admin_id: String::from(admin_id),
            player_ids: BTreeSet::new(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn player_ids(&self) -> &BTreeSet<String> {
        &self.player_ids
    }

    pub fn admin_id(&self) -> &str {
        &self.admin_id
    }

    pub fn add_player(&mut self, player_id: &str) {
        self.player_ids.insert(String::from(player_id));
    }

    pub fn remove_player(&mut self, player_id: &str) {
        if player_id == self.admin_id {
            // try to make next player to new admin
            match self.player_ids.iter().next() {
                Some(new_admin_id) => {
                    self.admin_id = String::from(new_admin_id);
                    self.player_ids.remove(&self.admin_id);
                }
                None => {}
            }
        } else if self.player_ids.contains(player_id) {
            self.player_ids.remove(player_id);
        }
    }
}

impl Persist for Game {
    fn id(&self) -> &str {
        self.token()
    }
}

impl Into<IVec> for Game {
    fn into(self) -> IVec {
        IVec::from(bincode::serialize(&self).unwrap())
    }
}

impl TryFrom<IVec> for Game {
    type Error = bincode::Error;
    fn try_from(bytes: IVec) -> Result<Self, Self::Error> {
        let vec: Vec<u8> = bytes.to_vec();

        bincode::deserialize(&vec)
    }
}
