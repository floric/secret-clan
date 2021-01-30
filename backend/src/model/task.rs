use super::{
    proto::{self},
    Party, Player, Voting,
};
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

impl From<proto::task::Task> for TaskDefinition {
    fn from(proto_task: proto::task::Task) -> Self {
        match proto_task.definition {
            Some(def) => match def {
                proto::task::Task_oneof_definition::settings(_) => TaskDefinition::Settings {},
                proto::task::Task_oneof_definition::discloseRole(_) => {
                    TaskDefinition::DiscloseRole {
                        role: Role::new("asd", Party::Good, "asd"),
                    }
                }
                proto::task::Task_oneof_definition::discuss(_) => TaskDefinition::Discuss {
                    time_limit: Utc::now(),
                },
                proto::task::Task_oneof_definition::vote(_) => TaskDefinition::Vote {
                    voting: Voting::new("asd", &vec![], &vec![]),
                },
            },
            None => TaskDefinition::DiscloseRole {
                role: Role::new("asd", Party::Good, "asd"),
            },
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
            TaskDefinition::DiscloseRole { role } => {
                task.set_discloseRole(proto::task::Task_DiscloseRole::new());
            }
            TaskDefinition::Discuss { time_limit } => {
                task.set_discuss(proto::task::Task_Discuss::new());
            }
            TaskDefinition::Vote { voting } => {
                task.set_vote(proto::task::Task_Vote::new());
            }
        }
        task
    }
}
