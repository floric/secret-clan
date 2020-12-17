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
    admin_id: Option<String>,
    player_ids: BTreeSet<String>,
}

impl Game {
    pub fn new(admin_id: &str, token: &str) -> Game {
        Game {
            token: String::from(token).to_uppercase(),
            creation_time: Utc::now(),
            last_action_time: Utc::now(),
            admin_id: Some(String::from(admin_id)),
            player_ids: BTreeSet::new(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn player_ids(&self) -> &BTreeSet<String> {
        &self.player_ids
    }

    pub fn admin_id(&self) -> &Option<String> {
        &self.admin_id
    }

    pub fn add_player(&mut self, player_id: &str) {
        match self.admin_id {
            Some(_) => {
                self.player_ids.insert(String::from(player_id));
            }
            None => self.admin_id = Some(String::from(player_id)),
        }
    }

    pub fn remove_player(&mut self, player_id: &str) {
        if self.player_ids.contains(player_id) {
            self.player_ids.remove(player_id);
        } else {
            match self.admin_id.to_owned().filter(|id| id == player_id) {
                Some(_) => {
                    if self.player_ids.iter().next().is_some() {
                        let next_player_id = self.player_ids.iter().next().unwrap().to_owned();
                        self.admin_id = Some(String::from(&next_player_id));
                        self.player_ids.remove(&next_player_id);
                    } else {
                        // abandon game as admin is the last player
                        self.admin_id = None;
                    }
                }
                None => {
                    // game is already abandoned or requesting user is no admin or player
                }
            }
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
