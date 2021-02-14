use crate::{
    model::{
        proto::{self},
        Player,
    },
    server::app_context::AppContext,
};
use chrono::{Duration, Utc};
use log::{debug, error, info, warn};

pub fn cleanup_players(ctx: &'static AppContext) -> impl Fn() {
    move || {
        tokio::spawn(async move {
            execute_cleanup_players(ctx, Duration::minutes(1)).await;
        });
    }
}

// Player is active after one minute without an active connection
fn is_inactive_player(duration: Duration) -> impl Fn(&Player) -> bool {
    let threshold = Utc::now().checked_sub_signed(duration).unwrap();

    move |player: &Player| {
        player.last_active_time().is_some() && player.last_active_time().unwrap().lt(&threshold)
    }
}

async fn execute_cleanup_players(ctx: &AppContext, duration: Duration) -> bool {
    let inactive_players = ctx
        .db()
        .players()
        .scan(Box::new(is_inactive_player(duration)))
        .await
        .expect("Scanning players has failed");
    let inactive_count = inactive_players.len();

    // remove players from maybe existing game
    for id in inactive_players.clone() {
        let player = ctx.db().players().get(&id).await;
        if let Some(player) = player.expect("Reading player has failed") {
            let game = ctx.db().games().get(player.game_token()).await;
            if let Some(mut game) = game.expect("Reading game has failed") {
                game.remove_player(&id);
                if ctx.db().games().persist(&game).await.is_err() {
                    warn!("Removing player has failed");
                }
                // inform remaining players about left player
                for remaining_id in game.all_player_ids().iter().filter(|p_id| **p_id != id) {
                    let mut player_left_msg = proto::message::Server_PlayerLeft::new();
                    player_left_msg.set_player_id(id.clone());
                    let mut msg = proto::message::Server::new();
                    msg.set_playerLeft(player_left_msg);
                    if let Err(err) = ctx.ws().send_message(String::from(remaining_id), msg).await {
                        error!(
                            "Informing {} about left {} failed: {:}",
                            remaining_id, &id, err
                        );
                    }
                }
            }
        }
    }

    // remove players
    if inactive_count > 0 {
        match ctx.db().players().remove_batch(&inactive_players).await {
            Ok(_) => {
                info!("Removed {} inactive players", inactive_count);
                true
            }
            Err(e) => {
                warn!(
                    "Removing {} inactive players has failed: {:?}",
                    inactive_count, e
                );
                false
            }
        }
    } else {
        debug!("Removed no inactive players");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::execute_cleanup_players;
    use crate::{
        model::{Game, GameState, Player},
        server::app_context::AppContext,
    };
    use chrono::Duration;

    #[tokio::test]
    async fn should_cleanup_player() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.set_inactive();
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player failed");
        let game = Game::new(player.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game failed");

        let res = execute_cleanup_players(&ctx, Duration::nanoseconds(1)).await;
        assert!(res);

        assert!(ctx.db().players().get(player.id()).await.unwrap().is_none());
        assert!(ctx
            .db()
            .games()
            .get(player.game_token())
            .await
            .unwrap()
            .expect("Game should still exist")
            .admin_id()
            .is_none());
        assert_eq!(
            ctx.db()
                .games()
                .get(player.game_token())
                .await
                .unwrap()
                .expect("Game should still exist")
                .state(),
            &GameState::Abandoned
        );
    }

    #[tokio::test]
    async fn should_not_cleanup_player() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player failed");

        let res = execute_cleanup_players(&ctx, Duration::minutes(1)).await;
        assert!(!res);

        assert!(ctx.db().players().get(player.id()).await.unwrap().is_some());
    }
}
