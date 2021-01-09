use crate::{
    model::{IncomingMessage, OutgoingMessage},
    server::{app_context::AppContext, auth::extract_verified_player},
};
use futures::StreamExt;
use log::{error, info};
use nanoid::nanoid;

pub fn handle_ws_connection(ws: warp::ws::Ws, ctx: &'static AppContext) -> impl warp::Reply {
    ws.on_upgrade(move |socket| async move {
        let (sender, mut receiver) = socket.split();
        let peer_id = nanoid!();
        ctx.add_connection(&peer_id, sender).await;

        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_close() {
                        unimplemented!("Close connection");
                    } else if msg.is_text() {
                        match msg.to_str() {
                            Ok(content) => {
                                info!("Received answer: {:?}", content);

                                match serde_json::from_str(content) {
                                    Ok(parsed) => {
                                        handle_incoming_message(parsed, ctx, &peer_id).await;
                                    }
                                    Err(err) => {
                                        error!("Parsing message has failed: {:?}", err);
                                    }
                                }
                            }
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
    })
}

async fn handle_incoming_message(message: IncomingMessage, ctx: &AppContext, peer_id: &str) {
    match message {
        IncomingMessage::Auth { token } => {
            match extract_verified_player(&token, ctx).await {
                Some(_) => {
                    if let Err(err) = ctx.send_message(OutgoingMessage::Welcome {}, peer_id).await {
                        error!("Sending Welcome to {} has failed: {}", peer_id, &err);
                    }
                }
                None => {
                    error!("Unauthorized user");
                }
            };
        }
    }
}
