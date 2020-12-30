use crate::{
    model::{Player, Task, TaskType},
    server::app_context::AppContext,
};
use async_trait::async_trait;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscloseRoleResult {
    acknowledge: bool,
}

#[async_trait]
impl Task for DiscloseRoleResult {
    fn get_type(&self) -> TaskType {
        TaskType::DiscloseRole
    }

    async fn apply_result(&self, player: &mut Player, _: &AppContext) -> Result<(), String> {
        if self.acknowledge == false {
            // TODO Maybe cancel game in case a player doesn't understand role
            info!(
                "Player {} hasn't acknowledged the role. The game needs to be cancelled.",
                player.id()
            );
        }
        Ok(())
    }

    fn resolve_after_first_answer(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::DiscloseRoleResult;
    use crate::{
        model::{Party, Player, Role, TaskDefinition},
        server::{app_context::AppContext, auth::generate_jwt_token, endpoints::tasks::apply_task},
    };
    use warp::{hyper::StatusCode, Reply};

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[tokio::test]
    async fn should_disclose_role_acknowledge() {
        let ctx = init_ctx();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::DiscloseRole {
            role: Role::new("Test", Party::Bad),
        });
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            DiscloseRoleResult { acknowledge: true },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_disclose_role_and_decline() {
        let ctx = init_ctx();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::DiscloseRole {
            role: Role::new("Test", Party::Good),
        });
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);

        let res = apply_task(
            DiscloseRoleResult { acknowledge: false },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);
    }
}
