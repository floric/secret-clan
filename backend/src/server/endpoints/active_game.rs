use super::client_messages::{handle_auth_confirmation, handle_game_start, handle_name_update};
use crate::{
    model::{
        proto::{self},
        Player,
    },
    server::app_context::AppContext,
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
        on_upgrade(socket, ctx).await;
    })
}

async fn on_upgrade(socket: WebSocket, ctx: &AppContext) {
    let (sender, mut receiver) = socket.split();

    match prepare_new_connection(ctx, sender).await {
        Ok(peer_id) => {
            // incoming message loop
            while let Some(msg) = receiver.next().await {
                process_incoming_message(msg, &peer_id, ctx).await;
            }

            // inform other players about left player
            if let Some(player_id) = ctx.ws().get_authenticated_player_for_peer(&peer_id).await {
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
                            if player.id() == player_id {
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

    let player = get_corresponding_player(peer_id, ctx).await;
    match message {
        proto::message::Client_oneof_message::authConfirmed(
            proto::message::Client_AuthConfirmed { token, .. },
        ) => handle_auth_confirmation(&token, peer_id, ctx).await,
        proto::message::Client_oneof_message::nameUpdated(ev) => {
            handle_name_update(&ev.name, player, ctx).await
        }
        proto::message::Client_oneof_message::gameStarted(_) => {
            handle_game_start(player, ctx).await
        }
    }
}

async fn get_corresponding_player(peer_id: &str, ctx: &AppContext) -> Option<Player> {
    let player_id = ctx.ws().get_authenticated_player_for_peer(peer_id).await;
    if !player_id.is_some() {
        return None;
    }
    return ctx
        .db()
        .players()
        .get(&player_id.unwrap())
        .await
        .ok()
        .and_then(|p| p);
}

// TODO add tests
