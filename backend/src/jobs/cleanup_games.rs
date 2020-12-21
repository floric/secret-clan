use crate::{db::Client, model::game::Game, server::app_context::AppContext};
use chrono::{Duration, Utc};
use log::{debug, info, warn};

pub fn cleanup_games(ctx: &'static AppContext) -> impl Fn() {
    move || {
        tokio::task::spawn(async move {
            execute_cleanup_games(ctx.db().games(), Duration::minutes(5)).await;
        });
    }
}

async fn execute_cleanup_games(client: &Client<Game>, duration: Duration) -> bool {
    let inactive_games = client
        .scan(Box::new(is_inactive_game(duration)))
        .await
        .expect("Scanning games has failed");

    let inactive_count = inactive_games.len();
    if inactive_count == 0 {
        debug!("Removed no inactive games");
        false
    } else {
        match client.remove_batch(&inactive_games).await {
            Ok(_) => {
                info!("Removed {} inactive games", inactive_count);
                true
            }
            Err(e) => {
                warn!(
                    "Removing {} inactive games has failed: {:?}",
                    inactive_count, e
                );
                false
            }
        }
    }
}

// Game is inactive if no admin is present and last activity happened five minutes ago
fn is_inactive_game(duration: Duration) -> impl Fn(&Game) -> bool {
    let threshold = Utc::now().checked_sub_signed(duration).unwrap();
    move |game: &Game| match game.admin_id() {
        Some(_) => false,
        None => game.last_action_time().lt(&threshold),
    }
}

#[cfg(test)]
mod tests {
    use super::execute_cleanup_games;
    use crate::{
        db::{Client, Database},
        model::game::Game,
    };
    use chrono::Duration;

    fn init_client() -> Client<Game> {
        let mut repo = Database::init("games");

        let sender = repo.sender();
        tokio::task::spawn(async move {
            repo.start_listening().await;
        });

        let client = Client::new(sender);

        client
    }

    #[tokio::test]
    async fn should_cleanup_games() {
        let client = init_client();
        let mut game = Game::new("admin", "TOKEN");
        game.remove_player("admin");
        client.persist(&game).await.expect("Game persist failed");
        assert!(client
            .get("TOKEN")
            .await
            .unwrap()
            .unwrap()
            .admin_id()
            .is_none());
        assert!(client.get("TOKEN").await.unwrap().is_some());

        let res = execute_cleanup_games(&client, Duration::nanoseconds(1)).await;
        assert!(res);

        assert!(client.get("TOKEN").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn should_not_cleanup_games_with_admin() {
        let client = init_client();
        let game = Game::new("admin", "TOKEN");
        client.persist(&game).await.expect("Game persist failed");
        assert!(client
            .get("TOKEN")
            .await
            .unwrap()
            .unwrap()
            .admin_id()
            .is_some());

        let res = execute_cleanup_games(&client, Duration::nanoseconds(1)).await;
        assert!(!res);

        assert!(client.get("TOKEN").await.unwrap().is_some());
    }

    #[tokio::test]
    async fn should_not_cleanup_games_with_recent_action() {
        let client = init_client();
        let mut game = Game::new("admin", "TOKEN");
        game.remove_player("admin");
        client.persist(&game).await.expect("Game persist failed");
        assert!(client
            .get("TOKEN")
            .await
            .unwrap()
            .unwrap()
            .admin_id()
            .is_none());

        let res = execute_cleanup_games(&client, Duration::minutes(5)).await;
        assert!(!res);

        assert!(client.get("TOKEN").await.unwrap().is_some());
    }
}
