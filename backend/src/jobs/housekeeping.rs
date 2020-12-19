use chrono::{Duration, Utc};
use log::{info, warn};

use crate::{model::player::Player, server::app_context::AppContext};

pub fn cleanup_games(_: &'static AppContext) -> impl Fn() {
    || {
        tokio::spawn(async move {
            info!("Cleaned games successfully");
        });
    }
}

pub fn cleanup_players(ctx: &'static AppContext) -> impl Fn() {
    let cleanup_periodically = move || {
        tokio::spawn(async move {
            let inactive_players = ctx
                .db()
                .players()
                .scan(scan_inactive_players)
                .await
                .expect("Scanning players has failed");
            let inactive_count = inactive_players.len();
            for id in inactive_players {
                let player = ctx.db().players().find_by_id(&id).await;
                match player.expect("Reading player has failed") {
                    Some(player) => {
                        // remove player from maybe existing game
                        let game = ctx.db().games().find_by_id(player.game_token()).await;
                        match game.expect("Reading game has failed") {
                            Some(mut game) => {
                                game.remove_player(&id);
                                let _ = ctx.db().games().persist(&game).await;
                            }
                            None => {}
                        }

                        // remove player itself
                        match ctx.db().players().remove(&id).await {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Cleanup of players failed: {}", e);
                            }
                        }
                    }
                    None => {}
                }
            }
            if inactive_count > 0 {
                info!("Removed {} inactive players", inactive_count);
            }
        });
    };

    cleanup_periodically
}

fn scan_inactive_players(player: &Player) -> bool {
    match Utc::now().checked_sub_signed(Duration::minutes(1)) {
        Some(threshold) => player.last_action_time().lt(&threshold),
        None => false,
    }
}
