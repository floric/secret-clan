use super::TaskDefinition;
use crate::{
    db::Persist,
    model::proto::{self},
};
use chrono::{DateTime, Utc};
use log::warn;
use names::Generator;
use nanoid::nanoid;
use protobuf::RepeatedField;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::collections::VecDeque;

fn generate_random_name() -> String {
    Generator::default().next().unwrap()
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    id: String,
    name: String,
    game_token: String,
    #[derivative(Debug = "ignore")]
    user_token: String,
    creation_time: DateTime<Utc>,
    last_active_time: Option<DateTime<Utc>>,
    open_tasks: VecDeque<TaskDefinition>,
    credits: u32,
    // TODO cards
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    id: String,
    name: String,
    game_token: String,
}

impl Player {
    pub fn new(game_token: &str) -> Self {
        Player {
            id: nanoid!(),
            name: generate_random_name(),
            game_token: String::from(game_token),
            user_token: String::from(""),
            creation_time: Utc::now(),
            last_active_time: None,
            open_tasks: VecDeque::default(),
            credits: 0,
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

    pub fn user_token(&self) -> &str {
        &self.user_token
    }

    pub fn update_token(&mut self, new_token: &str) {
        self.user_token = String::from(new_token);
    }

    pub fn set_inactive(&mut self) {
        self.last_active_time = Some(Utc::now());
    }

    pub fn set_active(&mut self) {
        self.last_active_time = None;
    }

    pub fn last_active_time(&self) -> Option<DateTime<Utc>> {
        self.last_active_time
    }

    pub fn assign_task(&mut self, task: TaskDefinition) {
        self.open_tasks.push_back(task);
    }

    pub fn resolve_task(&mut self, task: TaskDefinition) {
        if self.open_tasks.front().filter(|t| **t == task).is_some() {
            self.open_tasks.pop_front();
        } else {
            warn!("Task {:?} not resolved", task);
        }
    }

    pub fn open_tasks(&self) -> &VecDeque<TaskDefinition> {
        &self.open_tasks
    }

    pub fn set_credits(&mut self, credits: u32) {
        self.credits = credits;
    }

    pub fn credits(&self) -> u32 {
        self.credits
    }
    pub fn to_response(&self) -> PlayerResponse {
        PlayerResponse {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            game_token: self.game_token.to_owned(),
        }
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

impl Into<proto::player::Player> for Player {
    fn into(self) -> proto::player::Player {
        let mut player = proto::player::Player::new();
        player.set_id(self.id);
        player.set_name(self.name);
        player.set_credits(self.credits);
        player
    }
}

impl Into<proto::player::OwnPlayer> for Player {
    fn into(self) -> proto::player::OwnPlayer {
        let mut player = proto::player::OwnPlayer::new();
        player.set_id(self.id);
        player.set_name(self.name);
        let mut open_tasks = RepeatedField::new();
        for t in self.open_tasks {
            open_tasks.push(t.into());
        }
        player.set_open_tasks(open_tasks);
        player.set_credits(self.credits);
        player
    }
}
