use crate::{model::game::Game, server::app_context::AppContext};
use chrono::{Duration, Utc};
use log::{debug, info, warn};

pub fn cleanup_games(ctx: &'static AppContext) -> impl Fn() {
    move || {
        tokio::task::spawn(async move {
            let inactive_games = ctx
                .db()
                .games()
                .scan(is_inactive_game)
                .await
                .expect("Scanning games has failed");
            let inactive_count = inactive_games.len();
            if inactive_count > 0 {
                match ctx.db().games().remove_batch(&inactive_games).await {
                    Ok(_) => {
                        info!("Removed {} inactive games", inactive_count);
                    }
                    Err(e) => {
                        warn!(
                            "Removing {} inactive games has failed: {:?}",
                            inactive_count, e
                        );
                    }
                }
            } else {
                debug!("Removed no inactive games");
            }
        });
    }
}

// Game is inactive if no admin is present and last activity happened five minutes ago
fn is_inactive_game(game: &Game) -> bool {
    match game.admin_id() {
        Some(_) => false,
        None => match Utc::now().checked_sub_signed(Duration::minutes(5)) {
            Some(threshold) => game.last_action_time().lt(&threshold),
            None => false,
        },
    }
}
