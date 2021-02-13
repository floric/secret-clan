use crate::{model::Player, server::app_context::AppContext};
use log::debug;

pub async fn handle_name_update(
    new_name: &str,
    player: Option<Player>,
    ctx: &AppContext,
) -> Result<(), String> {
    if let Some(mut player) = player {
        player.set_name(new_name);
        return match ctx.db().players().persist(&player).await {
            Ok(_) => {
                debug!("Applied settings player {}", player.id());
                Ok(())
            }
            Err(err) => Err(std::fmt::format(format_args!(
                "Writing name of player {} has failed: {:?}",
                player.id(),
                err
            ))),
        };
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::handle_name_update;
    use crate::{model::Player, server::app_context::AppContext};

    #[tokio::test]
    async fn should_do_nothing_for_missing_person() {
        let ctx = AppContext::init();

        let res = handle_name_update("Test", None, &ctx).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn should_rename() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        let player_id = String::from(player.id());
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        ctx.ws()
            .register_active_player(player.id(), "peer")
            .await
            .expect("Setting peer connection failed");

        let res = handle_name_update("Test", Some(player), &ctx).await;
        assert!(res.is_ok());

        let updated_player = ctx
            .db()
            .players()
            .get(&player_id)
            .await
            .expect("Reading player has failed")
            .unwrap();
        assert_eq!(updated_player.name(), "Test");
    }
}
