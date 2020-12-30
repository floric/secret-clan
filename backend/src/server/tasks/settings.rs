use super::Task;
use crate::{
    model::{Player, Tasks},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SettingsResult {
    pub name: String,
}

pub struct SettingsTask {}

#[async_trait]
impl Task<SettingsResult> for SettingsTask {
    fn get_type(&self) -> Tasks {
        Tasks::Settings {}
    }

    async fn apply_result(
        &self,
        res: SettingsResult,
        player: &mut Player,
        ctx: &AppContext,
    ) -> Result<bool, String> {
        player.set_name(&res.name);
        match ctx.db().players().persist(player).await {
            Ok(_) => {
                debug!("Applied settings player {}", player.id());
                Ok(true)
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
        model::{Player, Tasks},
        server::{
            app_context::AppContext,
            auth::generate_jwt_token,
            endpoints::tasks::apply_task,
            tasks::settings::{SettingsResult, SettingsTask},
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
        player.add_task(Tasks::Settings {});
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            SettingsTask {},
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
