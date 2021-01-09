use crate::{
    model::{IncomingMessage, OutgoingMessage},
    server::{app_context::AppContext, auth::extract_verified_player},
};
use futures::{stream::SplitSink, StreamExt};
use log::{debug, error};
use nanoid::nanoid;
use warp::ws::{Message, WebSocket};

pub fn handle_ws_filter(ws: warp::ws::Ws, ctx: &'static AppContext) -> impl warp::Reply {
    ws.on_upgrade(move |socket| async move {
        let (sender, mut receiver) = socket.split();

        match prepare_new_connection(ctx, sender).await {
            Ok(peer_id) => {
                while let Some(msg) = receiver.next().await {
                    match msg {
                        Ok(msg) => {
                            if msg.is_close() {
                                debug!("Connection to {} closed", &peer_id);
                            } else if msg.is_text() {
                                match msg.to_str() {
                                    Ok(content) => match serde_json::from_str(content) {
                                        Ok(parsed) => {
                                            if let Err(err) =
                                                handle_incoming_message(parsed, ctx, &peer_id).await
                                            {
                                                error!("Handling message has failed: {}", err);
                                            }
                                        }
                                        Err(err) => {
                                            error!("Parsing message has failed: {:?}", err);
                                        }
                                    },
                                    Err(_) => {
                                        error!("Reading message has failed");
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            error!("Receiving message has failed: {:?}", err);
                        }
                    }
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
    sender: SplitSink<WebSocket, Message>,
) -> Result<String, String> {
    let peer_id = nanoid!();

    ctx.ws().add_connection(&peer_id, sender).await?;

    Ok(peer_id)
}

async fn handle_incoming_message(
    message: IncomingMessage,
    ctx: &AppContext,
    peer_id: &str,
) -> Result<(), String> {
    debug!("Received message: {:?}", message);

    match message {
        IncomingMessage::Auth { token } => match extract_verified_player(&token, ctx).await {
            Some(player) => {
                ctx.ws()
                    .register_active_player(player.id(), peer_id)
                    .await
                    .expect("Registering player has failed");
                if let Some(next_task) = player.open_tasks().front() {
                    return ctx
                        .ws()
                        .send_message(
                            player.id(),
                            OutgoingMessage::NewTask {
                                task: next_task.clone(),
                            },
                        )
                        .await;
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
        model::{IncomingMessage, Player},
        server::{app_context::AppContext, auth::generate_jwt_token},
    };

    #[tokio::test]
    async fn should_handle_auth_message_with_open_task() {
        let ctx = AppContext::init();
        let mut player = Player::new("GAME");
        player.assign_task(crate::model::TaskDefinition::Settings {});
        ctx.db()
            .players()
            .persist(&player)
            .await
            .expect("Persisting player has failed");
        let token = generate_jwt_token(&player, &ctx.config().auth_secret);

        let reply = handle_incoming_message(IncomingMessage::Auth { token }, &ctx, "peer-id").await;

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

        let reply = handle_incoming_message(IncomingMessage::Auth { token }, &ctx, "peer-id").await;

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
            IncomingMessage::Auth {
                token: String::from("invalid"),
            },
            &ctx,
            "peer-id",
        )
        .await;

        assert_eq!(reply.unwrap_err(), "Unauthorized user");
    }
}
