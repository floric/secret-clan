use crate::{model::Player, server::app_context::AppContext};
use log::error;
use tokio::sync::mpsc;

pub struct ChangeListener {
    players: mpsc::Receiver<Player>,
}

impl ChangeListener {
    pub fn new(players: mpsc::Receiver<Player>) -> Self {
        ChangeListener { players }
    }

    pub async fn start_listening(&mut self, ctx: &AppContext) {
        while let Some(player) = self.players.recv().await {
            match ctx.db().games().get(player.game_token()).await {
                Ok(game) => {
                    if let Some(game) = game {
                        // inform all players of game about updated player
                        for player_id in game.all_player_ids() {
                            if let Err(err) = ctx
                                .ws()
                                .send_message(
                                    player_id,
                                    crate::model::OutgoingMessage::PlayerUpdated {
                                        player: player.clone(),
                                    },
                                )
                                .await
                            {
                                error!("Sending PlayerUpdated has failed: {}", &err);
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{Game, Player},
        server::app_context::AppContext,
    };

    #[tokio::test]
    async fn should_get_game() {
        let (ctx, mut changes) = AppContext::init_with_changes();
        let ctx: &'static AppContext = Box::leak(Box::new(ctx));
        tokio::spawn(async move {
            changes.start_listening(&ctx).await;
        });

        let admin = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&admin)
            .await
            .expect("Writing admin failed");

        let game = Game::new(admin.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Writing game failed");

        let mut player = Player::new("GAME");
        let res = ctx.db().players().persist(&player).await;
        assert!(res.is_ok());

        player.set_name("new name");

        let res = ctx.db().players().persist(&player).await;
        assert!(res.is_ok());
    }
}
