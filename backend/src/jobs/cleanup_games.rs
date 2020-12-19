use crate::{model::game::Game, server::app_context::AppContext};
use chrono::{Duration, Utc};
use log::{debug, info};

pub fn cleanup_games(ctx: &'static AppContext) -> impl Fn() {
    let cleanup_periodically = move || {
        tokio::task::spawn(async move {
            let inactive_games = ctx
                .db()
                .games()
                .scan(is_inactive_game)
                .await
                .expect("Scanning games has failed");
            let inactive_count = inactive_games.len();
            for id in inactive_games {
                let _ = ctx.db().games().remove(&id).await;
            }
            if inactive_count > 0 {
                info!("Removed {} inactive games", inactive_count);
            } else {
                debug!("Removed no inactive games");
            }
        });
    };

    cleanup_periodically
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
