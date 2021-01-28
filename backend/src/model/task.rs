use super::{Player, Voting};
use crate::{model::Role, server::app_context::AppContext};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    Settings,
    DiscloseRole,
    Discuss,
    Vote,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskDefinition {
    Settings {},
    DiscloseRole { role: Role },
    Discuss { time_limit: DateTime<Utc> },
    Vote { voting: Voting },
}

impl TaskDefinition {
    pub fn get_type(&self) -> TaskType {
        match self {
            TaskDefinition::Settings {} => TaskType::Settings,
            TaskDefinition::DiscloseRole { .. } => TaskType::DiscloseRole,
            TaskDefinition::Discuss { .. } => TaskType::Discuss,
            TaskDefinition::Vote { .. } => TaskType::Vote,
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
