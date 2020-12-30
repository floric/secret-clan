use crate::{
    model::{Player, Task, TaskType},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SettingsResult {
    pub name: String,
}

#[async_trait]
impl Task for SettingsResult {
    fn get_type(&self) -> TaskType {
        TaskType::Settings
    }

    async fn apply_result(&self, player: &mut Player, ctx: &AppContext) -> Result<(), String> {
        player.set_name(&self.name);
        match ctx.db().players().persist(player).await {
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
            app_context::AppContext, auth::generate_jwt_token, endpoints::tasks::apply_task,
            tasks::settings::SettingsResult,
        },
    };
    use warp::{hyper::StatusCode, Reply};

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_change_name() {
        let ctx = init_ctx();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::Settings {});
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            SettingsResult {
                name: String::from("Test"),
            },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);

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
