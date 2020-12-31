use crate::{
    model::{Player, Task, TaskType},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscussTask {}

#[async_trait]
impl Task for DiscussTask {
    fn get_type(&self) -> TaskType {
        TaskType::Discuss
    }

    async fn apply_result(&self, _: Player, _: &AppContext) -> Result<(), String> {
        Ok(())
    }

    fn resolve_after_first_answer(&self) -> bool {
        true
    }
}
