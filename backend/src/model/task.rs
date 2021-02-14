use super::proto::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskDefinition {
    Settings {},
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
