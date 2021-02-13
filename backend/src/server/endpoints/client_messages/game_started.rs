use crate::{
    model::{Player, TaskType},
    server::app_context::AppContext,
};
use std::fmt::format;

pub async fn handle_game_start(player: Option<Player>, ctx: &AppContext) -> Result<(), String> {
    match player {
        Some(player) => {
            match ctx
                .db()
                .games()
                .get(player.game_token())
                .await
                .expect("Reading game has failed")
                .filter(|game| game.admin_id().is_some())
                .filter(|game| player.id() == game.admin_id().as_ref().unwrap())
            {
                Some(mut game) => {
                    match ctx.db().players().get_batch(&game.all_player_ids()).await {
                        Ok(mut players) => {
                            game.start();
                            let players = players
                                .values_mut()
                                .map(|p| {
                                    p.resolve_task(TaskType::Settings);
                                    p.set_credits(5000);
                                    p.clone()
                                })
                                .collect::<Vec<_>>();
                            let (persist_players, persist_game) = tokio::join!(
                                ctx.db().players().persist_batch(&players),
                                ctx.db().games().persist(&game)
                            );
                            return persist_players
                                .and(persist_game)
                                .map_err(|_| String::from("Updating game or players failed"));
                        }
                        Err(err) => Err(format(format_args!(
                            "Persisting players has failed: {:?}",
                            err
                        ))),
                    }
                }
                None => Err(String::from("Game not found")),
            }
        }
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::handle_game_start;
    use crate::{
        model::{Game, Player},
        server::app_context::AppContext,
    };

    #[tokio::test]
    async fn should_start_game() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let game = Game::new(player.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game failed");
        ctx.ws()
            .register_active_player(player.id(), "peer")
            .await
            .expect("Registering player failed");

        let reply = handle_game_start(Some(player), &ctx).await;
        assert!(reply.is_ok());
    }

    #[tokio::test]
    async fn should_not_start_unknown_game() {
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
            .expect("Registering player failed");

        let reply = handle_game_start(Some(player), &ctx).await;
        assert!(reply.is_err());
    }

    #[tokio::test]
    async fn should_not_start_for_normal_player() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let admin = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&admin)
            .await
            .expect("Persisting player has failed");
        let game = Game::new(admin.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game failed");
        ctx.ws()
            .register_active_player(player.id(), "participant_peer")
            .await
            .expect("Registering player failed");

        let reply = handle_game_start(Some(player), &ctx).await;
        assert!(reply.is_err());
    }
}
