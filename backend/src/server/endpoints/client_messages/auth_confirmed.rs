use crate::{
    model::proto::{self},
    server::{app_context::AppContext, auth::extract_verified_player},
};
use log::{debug, error};

pub async fn handle_auth_confirmation(
    token: &str,
    peer_id: &str,
    ctx: &AppContext,
) -> Result<(), String> {
    match extract_verified_player(&token, ctx).await {
        Some(mut new_player) => {
            new_player.set_active();
            if let Err(err) = ctx.db().players().persist(&new_player).await {
                error!("Setting player active failed: {:?}", err);
            }
            ctx.ws()
                .register_active_player(new_player.id(), peer_id)
                .await
                .expect("Registering player has failed");

            if let Some(game) = ctx
                .db()
                .games()
                .get(new_player.game_token())
                .await
                .expect("Reading game has failed")
            {
                let mut game_updated_msg = proto::message::Server_GameUpdated::new();
                game_updated_msg.set_game(game.clone().into());
                let mut msg = proto::message::Server::new();
                msg.set_gameUpdated(game_updated_msg);
                ctx.ws()
                    .send_message(String::from(new_player.id()), msg)
                    .await?;

                for other_player_id in game.all_player_ids() {
                    if let Some(other_player) = ctx
                        .db()
                        .players()
                        .get(&other_player_id)
                        .await
                        .expect("Reading player has failed")
                    {
                        // inform new player about existing players
                        if other_player_id != new_player.id() {
                            let mut update_msg = proto::message::Server_PlayerEntered::new();
                            update_msg.set_player(other_player.clone().into());
                            let mut msg = proto::message::Server::new();
                            msg.set_playerEntered(update_msg);
                            ctx.ws()
                                .send_message(String::from(new_player.id()), msg)
                                .await?;
                        }

                        // inform other players about new player
                        let mut player_msg = proto::message::Server_PlayerEntered::new();
                        player_msg.set_player(new_player.clone().into());
                        let mut msg = proto::message::Server::new();
                        msg.set_playerEntered(player_msg);
                        ctx.ws()
                            .send_message(String::from(other_player.id()), msg)
                            .await?;
                    }
                }
            }
            Ok(())
        }
        None => {
            let mut msg = proto::message::Server::new();
            msg.set_gameDeclined(proto::message::Server_GameDeclined::new());
            ctx.ws().send_message(String::from(peer_id), msg).await?;
            debug!("Unauthorized user tried to access game {}", token);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::handle_auth_confirmation;
    use crate::{
        config::AppConfig,
        model::{ Game, Player, TaskDefinition},
        server::{
            app_context::{AppContext, DbClients},
            auth::generate_jwt_token,
            ws::{WsClient, WsCommand},
        },
    };
    use flexi_logger::Level;
    use log::error;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn should_handle_auth_message_with_open_task() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::Settings {});
        let game = Game::new(player.id(), "GAME");
        ctx.db()
            .games()
            .persist(&game)
            .await
            .expect("Persisting game has failed");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);
        ctx.ws()
            .register_active_player(player.id(), "peer-id")
            .await
            .expect("Registering players connection failed");

        let reply = handle_auth_confirmation(&token, "peer-id", &ctx).await;
        assert!(reply.is_ok());
    }

    #[tokio::test]
    async fn should_handle_auth_message_without_open_task() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);
        ctx.ws()
            .register_active_player(player.id(), "peer-id")
            .await
            .expect("Registering players connection failed");

        let reply = handle_auth_confirmation(&token, "peer-id", &ctx).await;
        assert!(reply.is_ok());
    }

    #[tokio::test]
    async fn should_handle_auth_message_with_invalid_token() {
        let (change_sender, mut change_receiver): (
            mpsc::Sender<WsCommand>,
            mpsc::Receiver<WsCommand>,
        ) = mpsc::channel(256);
        let ctx = AppContext {
            config: AppConfig {
                auth_secret: String::from("auth"),
                log_level: Level::Debug,
                port: 80,
            },
            db: DbClients::init(),
            ws: WsClient {
                sender: change_sender,
            },
        };
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");

        let reply = handle_auth_confirmation("invalid", "peer-id", &ctx).await;
        assert!(reply.is_ok());

        let sent_msg = change_receiver.recv().await;

        if let Some(command) = sent_msg {
            match command {
                WsCommand::SendMessage { msg, .. } => {
                    assert!(msg.has_gameDeclined());
                }
                _ => {
                    error!("Unexpected type: {:?}", command);
                }
            }
        } else {
            panic!("Received no command");
        }
    }
}
