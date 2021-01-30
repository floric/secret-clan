use crate::{
    model::proto::{self},
    server::{app_context::AppContext, auth::extract_verified_player},
};
use futures::{stream::SplitSink, StreamExt};
use log::{debug, error, warn};
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
                while let Some(msg) = receiver.next().await {
                    process_incoming_message(msg, &peer_id, ctx).await;
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
        proto::message::Client_oneof_message::auth(proto::message::Client_Auth {
            token, ..
        }) => match extract_verified_player(&token, ctx).await {
            Some(player) => {
                ctx.ws()
                    .register_active_player(player.id(), peer_id)
                    .await
                    .expect("Registering player has failed");
                if let Some(next_task) = player.open_tasks().front() {
                    let mut new_task_msg = proto::message::Server_NewTask::new();
                    new_task_msg.set_task(next_task.clone().into());

                    let mut msg = proto::message::Server::new();
                    msg.set_newTask(new_task_msg);
                    return ctx.ws().send_message(String::from(player.id()), msg).await;
                }
                Ok(())
            }
            None => Err(String::from("Unauthorized user")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::handle_incoming_message;
    use crate::{
        model::{
            proto::message::{Client_Auth, Client_oneof_message},
            Player, TaskDefinition,
        },
        server::{app_context::AppContext, auth::generate_jwt_token},
    };

    #[tokio::test]
    async fn should_handle_auth_message_with_open_task() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(TaskDefinition::Settings {});
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
            Client_oneof_message::auth(Client_Auth {
                token,
                ..Default::default()
            }),
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
            Client_oneof_message::auth(Client_Auth {
                token,
                ..Default::default()
            }),
            &ctx,
            "peer-id",
        )
        .await;

        assert!(reply.is_ok());
    }

    #[tokio::test]
    async fn should_handle_auth_message_with_invalid_token() {
        let ctx = AppContext::init();
        let player = Player::new("GAME");
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");

        let reply = handle_incoming_message(
            Client_oneof_message::auth(Client_Auth {
                token: String::from("invalid"),
                ..Default::default()
            }),
            &ctx,
            "peer-id",
        )
        .await;

        assert_eq!(reply.unwrap_err(), "Unauthorized user");
    }
}
