use crate::{
    model::{
        proto::{self},
        Player, Task, TaskDefinition, TaskType,
    },
    server::app_context::AppContext,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscloseRoleTask {
    acknowledge: bool,
}

#[async_trait]
impl Task for DiscloseRoleTask {
    fn get_type(&self) -> TaskType {
        TaskType::DiscloseRole
    }

    async fn apply_result(&self, mut player: Player, ctx: &AppContext) -> Result<(), String> {
        if !self.acknowledge {
            // TODO Maybe cancel game in case a player doesn't understand role
            info!(
                "Player {} hasn't acknowledged the role. The game needs to be cancelled.",
                player.id()
            );
        }

        player.acknowledge_role();

        if let Err(err) = ctx.db().players().persist(&player).await {
            return Err(err.to_string());
        }

        match ctx.db().games().get(player.game_token()).await {
            Ok(game) => match game {
                Some(game) => match ctx.db().players().get_batch(&game.all_player_ids()).await {
                    Ok(mut all_players) => {
                        let all_have_acknowledged_role =
                            all_players.values().all(|p| *p.acknowledged_role());

                        if all_have_acknowledged_role {
                            let time_limit = Utc::now()
                                .checked_add_signed(Duration::minutes(15))
                                .expect("Adding duration should succeed");
                            let new_task = TaskDefinition::Discuss { time_limit };
                            let updated_players = all_players
                                .values_mut()
                                .map(|p| {
                                    p.assign_task(new_task.clone());
                                    p.clone()
                                })
                                .collect::<Vec<_>>();
                            if ctx
                                .db()
                                .players()
                                .persist_batch(&updated_players)
                                .await
                                .is_err()
                            {
                                return Err(String::from("Adding discuss task has failed"));
                            }

                            let mut futures = vec![];
                            for p in updated_players {
                                let mut new_task_msg = proto::message::Server_NewTask::new();
                                new_task_msg
                                    .set_task(p.open_tasks().iter().next().unwrap().clone().into());

                                let mut msg = proto::message::Server::new();
                                msg.set_newTask(new_task_msg);

                                futures.push(ctx.ws().send_message(String::from(p.id()), msg));
                            }
                            return futures::future::try_join_all(futures).await.map(|_| ());
                        }

                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("Game associated with player not found")),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    fn resolve_after_first_answer(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::DiscloseRoleTask;
    use crate::{
        model::{Game, Party, Player, Role, TaskDefinition, TaskType},
        server::{app_context::AppContext, auth::generate_jwt_token, endpoints::tasks::apply_task},
    };
    use warp::{hyper::StatusCode, Reply};

    #[tokio::test]
    async fn should_disclose_role_and_acknowledge() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::DiscloseRole {
            role: Role::new("Test", Party::Bad, ""),
        });
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);
        let game = Game::new(player.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game has failed");

        let res = apply_task(DiscloseRoleTask { acknowledge: true }, &authorization, &ctx).await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);

        assert_eq!(
            ctx.db()
                .players()
                .get(player.id())
                .await
                .expect("Reading player has failed")
                .unwrap()
                .open_tasks()
                .iter()
                .next()
                .unwrap()
                .get_type(),
            TaskType::Discuss
        );
    }

    #[tokio::test]
    async fn should_disclose_role_and_decline() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::DiscloseRole {
            role: Role::new("Test", Party::Good, ""),
        });
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let authorization = generate_jwt_token(&player, &ctx.config().auth_secret);
        let game = Game::new(player.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game has failed");

        let res = apply_task(
            DiscloseRoleTask { acknowledge: false },
            &authorization,
            &ctx,
        )
        .await;
        assert_eq!(res.unwrap().into_response().status(), StatusCode::OK);
    }
}
