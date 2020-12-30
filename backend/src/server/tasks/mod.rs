pub mod settings;

use crate::{
    model::{Player, Tasks},
    server::app_context::AppContext,
};
use async_trait::async_trait;

#[async_trait]
pub trait Task<R> {
    /// Generates the content for the task.
    fn get_type(&self) -> Tasks;

    /// Applies the result of the users decision and might mutate the game state.
    async fn apply_result(
        &self,
        result: R,
        player: &mut Player,
        ctx: &AppContext,
    ) -> Result<bool, String>;

    /// Determines if this task can be applied multiple times.
    fn resolve_after_first_answer(&self) -> bool;
}
