use log::warn;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::collections::{HashMap, HashSet};

use crate::db::Persist;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Voting {
    id: String,
    name: String,
    description: String,
    remaining_players: HashSet<String>,
    options: Vec<VoteOption>,
    votes: HashMap<String, String>,
}

impl Voting {
    pub fn new(name: &str, player_ids: &[String], options: &[VoteOption]) -> Self {
        Voting {
            id: nanoid!(),
            name: String::from(name),
            description: String::from(""),
            remaining_players: player_ids.iter().map(String::from).collect::<HashSet<_>>(),
            options: Vec::from(options),
            votes: HashMap::default(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn add_vote(&mut self, player_id: &str, option: &str) {
        if self.remaining_players.contains(player_id) {
            self.remaining_players.remove(player_id);
            self.votes
                .insert(String::from(player_id), String::from(option));
        } else {
            warn!("Player is not allowed to vote");
        }
    }

    pub fn votes(&self) -> &HashMap<String, String> {
        &self.votes
    }
}

impl Persist for Voting {
    fn id(&self) -> &str {
        self.id()
    }
}

impl Into<IVec> for Voting {
    fn into(self) -> IVec {
        IVec::from(bincode::serialize(&self).unwrap())
    }
}

impl From<IVec> for Voting {
    fn from(bytes: IVec) -> Voting {
        let vec: Vec<u8> = bytes.to_vec();
        bincode::deserialize(&vec).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct VoteOption {
    id: String,
    label: String,
}

impl VoteOption {
    pub fn new(id: &str, label: &str) -> Self {
        VoteOption {
            id: String::from(id),
            label: String::from(label),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    option: String,
    player_id: String,
}
