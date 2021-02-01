use crate::{model::Task, server::app_context::AppContext};
use log::warn;
pub async fn apply_task<T: Task>(task: T, peer_id: &str, ctx: &AppContext) -> Result<(), String> {
    match ctx.ws().get_authenticated_player_for_peer(peer_id).await {
        Some(player_id) => match ctx
            .db()
            .players()
            .get(&player_id)
            .await
            .expect("Reading player has failed")
        {
            Some(player) => {
                // Check if task is assigned
                if player
                    .open_tasks()
                    .front()
                    .filter(|def| def.get_type() == task.get_type())
                    .is_none()
                {
                    // Prevent leaking information about assigned tasks of other players by sending still OK
                    warn!(
                        "Player {} doesn't have task {:?} to resolve",
                        player.id(),
                        task.get_type()
                    );
                    return Ok(());
                }
                let player_id = player.id().to_owned();
                match task.apply_result(player, ctx).await {
                    Ok(_) => {
                        if task.resolve_after_first_answer() {
                            let mut player = ctx
                                .db()
                                .players()
                                .get(&player_id)
                                .await
                                .expect("Loading player has failed")
                                .unwrap();
                            player.resolve_task(task.get_type());
                            if ctx.db().players().persist(&player).await.is_err() {
                                return Err(String::from("Updating player has failed"));
                            }
                        }

                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(String::from("Player not found")),
        },
        None => Err(String::from("Player not authenticated")),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::Player,
        server::{
            app_context::AppContext, endpoints::tasks::apply_task, tasks::settings::SettingsTask,
        },
    };

    #[tokio::test]
    async fn should_do_nothing_for_unassigned_tasks() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
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
        assert_eq!(updated_player.name(), player.name());
    }
}
