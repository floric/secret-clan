use super::{
    proto::{self},
    Player,
};
use crate::server::app_context::AppContext;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    Settings,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskDefinition {
    Settings {},
}

impl TaskDefinition {
    pub fn get_type(&self) -> TaskType {
        match self {
            TaskDefinition::Settings {} => TaskType::Settings,
        }
    }
}

#[async_trait]
pub trait Task {
    /// Returns the type of this task to connect them with open tasks associated to each player.
    fn get_type(&self) -> TaskType;

    /// Applies the result of the users decision and might mutate the game state.
    async fn apply_result(&self, mut player: Player, ctx: &AppContext) -> Result<(), String>;

    /// Determines if this task can be applied multiple times.
    fn resolve_after_first_answer(&self) -> bool;
}

impl From<proto::task::Task> for TaskDefinition {
    fn from(proto_task: proto::task::Task) -> Self {
        match proto_task.definition {
            Some(def) => match def {
                proto::task::Task_oneof_definition::settings(_) => TaskDefinition::Settings {},
            },
            None => TaskDefinition::Settings {},
        }
    }
}
impl Into<proto::task::Task> for TaskDefinition {
    fn into(self) -> proto::task::Task {
        let mut task = proto::task::Task::new();
        // TODO
        match self {
            TaskDefinition::Settings {} => {
                task.set_settings(proto::task::Task_Settings::new());
            }
        }
        task
    }
}
