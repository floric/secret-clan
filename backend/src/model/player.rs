use super::{TaskDefinition, TaskType};
use crate::db::Persist;
use chrono::{DateTime, Utc};
use log::warn;
use names::Generator;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{collections::VecDeque, hash::Hash};

fn generate_random_name() -> String {
    Generator::default().next().unwrap()
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    id: String,
    name: String,
    game_token: String,
    #[derivative(Debug = "ignore")]
    user_token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
    open_tasks: VecDeque<TaskDefinition>,
    acknowledged_role: bool,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Derivative)]
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
            last_action_time: Utc::now(),
            open_tasks: VecDeque::new(),
            acknowledged_role: false,
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

    pub fn acknowledged_role(&self) -> &bool {
        &self.acknowledged_role
    }

    pub fn acknowledge_role(&mut self) {
        self.acknowledged_role = true;
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

    pub fn assign_task(&mut self, task: TaskDefinition) {
        self.open_tasks.push_back(task);
    }

    pub fn resolve_task(&mut self, task: TaskType) {
        if self
            .open_tasks
            .front()
            .filter(|t| t.get_type() == task)
            .is_some()
        {
            self.open_tasks.pop_front();
        } else {
            warn!("Task {:?} not resolved", task);
        }
    }

    pub fn open_tasks(&self) -> &VecDeque<TaskDefinition> {
        &self.open_tasks
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
