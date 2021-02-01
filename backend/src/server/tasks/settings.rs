use crate::{
    model::{Player, Task, TaskType},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SettingsTask {
    pub name: String,
}

#[async_trait]
impl Task for SettingsTask {
    fn get_type(&self) -> TaskType {
        TaskType::Settings
    }

    async fn apply_result(&self, mut player: Player, ctx: &AppContext) -> Result<(), String> {
        player.set_name(&self.name);
        match ctx.db().players().persist(&player).await {
            Ok(_) => {
                debug!("Applied settings player {}", player.id());
                Ok(())
            }
            Err(err) => Err(std::fmt::format(format_args!(
                "Writing name of player {} has failed: {:?}",
                player.id(),
                err
            ))),
        }
    }

    fn resolve_after_first_answer(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{Player, TaskDefinition},
        server::{
            app_context::AppContext, endpoints::tasks::apply_task, tasks::settings::SettingsTask,
        },
    };

    #[tokio::test]
    async fn should_change_name() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::Settings {});
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        ctx.ws()
            .register_active_player(player.id(), "peer")
            .await
            .expect("Setting peer connection failed");

        let res = apply_task(
            SettingsTask {
                name: String::from("Test"),
            },
            "peer",
            &ctx,
        )
        .await;
        assert!(res.is_ok());

        let updated_player = ctx
            .db()
            .players()
            .get(player.id())
            .await
            .expect("Reading player has failed")
            .unwrap();
        assert_eq!(updated_player.name(), "Test");
    }
}
