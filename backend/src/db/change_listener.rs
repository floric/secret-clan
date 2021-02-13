use crate::{
    model::{
        proto::{self},
        Game, Player,
    },
    server::app_context::AppContext,
};
use log::error;
use tokio::sync::mpsc;

pub struct ChangeListener {
    players: mpsc::Receiver<Player>,
    games: mpsc::Receiver<Game>,
}

impl ChangeListener {
    pub fn new(players: mpsc::Receiver<Player>, games: mpsc::Receiver<Game>) -> Self {
        ChangeListener { players, games }
    }

    pub async fn start_listening(&mut self, ctx: &AppContext) {
        tokio::join!(
            ChangeListener::listen_to_players(&mut self.players, ctx),
            ChangeListener::listen_to_games(&mut self.games, ctx)
        );
    }

    async fn listen_to_players(updated_players: &mut mpsc::Receiver<Player>, ctx: &AppContext) {
        while let Some(player) = updated_players.recv().await {
            if let Ok(Some(game)) = ctx.db().games().get(player.game_token()).await {
                // inform all players of game about updated player
                for player_id in game.all_player_ids() {
                    let mut msg = proto::message::Server::new();
                    if player_id.eq(player.id()) {
                        let mut update_msg = proto::message::Server_SelfUpdated::new();
                        update_msg.set_player(player.clone().into());
                        msg.set_selfUpdated(update_msg);
                    } else {
                        let mut update_msg = proto::message::Server_PlayerUpdated::new();
                        update_msg.set_player(player.clone().into());
                        msg.set_playerUpdated(update_msg);
                    }
                    if let Err(err) = ctx.ws().send_message(player_id, msg).await {
                        error!("Sending PlayerUpdated has failed: {}", &err);
                    }
                }
            }
        }
    }

    async fn listen_to_games(updated_games: &mut mpsc::Receiver<Game>, ctx: &AppContext) {
        while let Some(game) = updated_games.recv().await {
            // inform all players of game about updated game
            for player_id in game.all_player_ids() {
                let mut update_msg = proto::message::Server_GameUpdated::new();
                update_msg.set_game(game.clone().into());

                let mut msg = proto::message::Server::new();
                msg.set_gameUpdated(update_msg);

                if let Err(err) = ctx.ws().send_message(player_id, msg).await {
                    error!("Sending GameUpdated has failed: {}", &err);
                }
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
    async fn should_listen_to_game_changes() {
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
