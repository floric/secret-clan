use super::client_messages::handle_auth_confirmation;
use crate::{
    model::proto::{self},
    server::{
        app_context::AppContext, endpoints::tasks::apply_task, tasks::settings::SettingsTask,
    },
};
use futures::{stream::SplitSink, StreamExt};
use log::{debug, error, info, warn};
use nanoid::nanoid;
use protobuf::Message;
use warp::{
    ws::{Message as WsMessage, WebSocket},
    Error,
};

pub fn handle_ws_filter(ws: warp::ws::Ws, ctx: &'static AppContext) -> impl warp::Reply {
    ws.on_upgrade(move |socket| async move {
        let (sender, mut receiver) = socket.split();

        match prepare_new_connection(ctx, sender).await {
            Ok(peer_id) => {
                // incoming message loop
                while let Some(msg) = receiver.next().await {
                    process_incoming_message(msg, &peer_id, ctx).await;
                }

                // inform other players about left player
                if let Some(player_id) = ctx.ws().get_authenticated_player_for_peer(&peer_id).await
                {
                    if let Some(mut player) = ctx
                        .db()
                        .players()
                        .get(&player_id)
                        .await
                        .ok()
                        .and_then(|x| x)
                    {
                        player.set_inactive();
                        if let Err(err) = ctx.db().players().persist(&player).await {
                            error!("Setting player active failed: {:?}", err);
                        } else {
                            info!("Player {} has closed its connection", &player_id);
                        }
                        if let Some(game) = ctx
                            .db()
                            .games()
                            .get(player.game_token())
                            .await
                            .ok()
                            .and_then(|game| game)
                        {
                            for player_id in game.all_player_ids() {
                                // skip left player
                                if &player.id() == &player_id {
                                    continue;
                                }

                                // inform other players about new player
                                let mut player_msg = proto::message::Server_PlayerLeft::new();
                                player_msg.set_player_id(String::from(player.id()));
                                let mut msg = proto::message::Server::new();
                                msg.set_playerLeft(player_msg);
                                if let Err(err) = ctx.ws().send_message(player_id, msg).await {
                                    warn!("Informing about left player failed: {:?}", err);
                                }
                            }
                        }
                    }
                }

                if let Err(err) = ctx.ws().remove_connection(&peer_id).await {
                    error!("Removing closed connection failed: {:?}", &err);
                }
            }
            Err(err) => {
                error!("Preparing new connection has failed: {}", &err);
            }
        }
    })
}

async fn prepare_new_connection(
    ctx: &AppContext,
    sender: SplitSink<WebSocket, WsMessage>,
) -> Result<String, String> {
    let peer_id = nanoid!();

    ctx.ws().add_connection(&peer_id, sender).await?;

    Ok(peer_id)
}

async fn process_incoming_message(msg: Result<WsMessage, Error>, peer_id: &str, ctx: &AppContext) {
    match msg {
        Ok(msg) => {
            if msg.is_close() {
                debug!("Connection to {} closed", &peer_id);
            } else if msg.is_binary() {
                match proto::message::Client::parse_from_bytes(&msg.into_bytes()) {
                    Ok(res) => {
                        match res.message {
                            Some(x) => {
                                if let Err(err) = handle_incoming_message(x, ctx, &peer_id).await {
                                    error!("Sending message has failed: {:?}", &err);
                                }
                            }
                            None => {
                                warn!("Message was empty");
                            }
                        };
                    }
                    Err(err) => {
                        error!("Reading incoming message failed: {:?}", &err);
                    }
                };
            } else if msg.is_text() {
                warn!("Textual protobuf message ignored, only binary format supported");
            }
        }
        Err(err) => {
            error!("Receiving message has failed: {:?}", err);
        }
    }
}

async fn handle_incoming_message(
    message: proto::message::Client_oneof_message,
    ctx: &AppContext,
    peer_id: &str,
) -> Result<(), String> {
    debug!("Received message: {:?}", message);

    match message {
        proto::message::Client_oneof_message::authConfirmed(
            proto::message::Client_AuthConfirmed { token, .. },
        ) => handle_auth_confirmation(&token, peer_id, ctx).await,
        proto::message::Client_oneof_message::nameUpdated(ev) => {
            apply_task(SettingsTask { name: ev.name }, peer_id, ctx).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::handle_incoming_message;
    use crate::{
        config::AppConfig,
        model::{
            proto::{self},
            Game, Player, TaskDefinition,
        },
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

        let reply = handle_incoming_message(
            proto::message::Client_oneof_message::authConfirmed(
                proto::message::Client_AuthConfirmed {
                    token,
                    ..Default::default()
                },
            ),
            &ctx,
            "peer-id",
        )
        .await;

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

        let reply = handle_incoming_message(
            proto::message::Client_oneof_message::authConfirmed(
                proto::message::Client_AuthConfirmed {
                    token,
                    ..Default::default()
                },
            ),
            &ctx,
            "peer-id",
        )
        .await;

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

        let reply = handle_incoming_message(
            proto::message::Client_oneof_message::authConfirmed(
                proto::message::Client_AuthConfirmed {
                    token: String::from("invalid"),
                    ..Default::default()
                },
            ),
            &ctx,
            "peer-id",
        )
        .await;

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
