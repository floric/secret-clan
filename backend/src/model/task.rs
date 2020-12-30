use super::Player;
use crate::{model::Role, server::app_context::AppContext};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    Settings,
    DiscloseRole,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskDefinition {
    Settings {},
    DiscloseRole { role: Role },
}

impl TaskDefinition {
    pub fn get_type(&self) -> TaskType {
        match self {
            TaskDefinition::Settings {} => TaskType::Settings,
            TaskDefinition::DiscloseRole { role: _ } => TaskType::DiscloseRole,
        }
    }
}

#[async_trait]
pub trait Task {
    /// Returns the type of this task to connect them with open tasks associated to each player.
    fn get_type(&self) -> TaskType;

    /// Applies the result of the users decision and might mutate the game state.
    async fn apply_result(&self, player: &mut Player, ctx: &AppContext) -> Result<(), String>;

    /// Determines if this task can be applied multiple times.
    fn resolve_after_first_answer(&self) -> bool;
}
