use super::WsCommand;
use crate::model::proto::{self};
use futures::stream::SplitSink;
use futures::SinkExt;
use log::{debug, error, info};
use protobuf::Message;
use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::ws::{Message as WsMessage, WebSocket};

pub struct Connections {
    connections: HashMap<String, SplitSink<WebSocket, WsMessage>>,
    player_to_peer: HashMap<String, String>,
    peer_to_player: HashMap<String, String>,
    msg_receiver: mpsc::Receiver<WsCommand>,
}

impl Connections {
    pub fn new() -> (Self, mpsc::Sender<WsCommand>) {
        let (msg_sender, msg_receiver): (mpsc::Sender<WsCommand>, mpsc::Receiver<WsCommand>) =
            mpsc::channel(128);

        let connections = Connections {
            connections: HashMap::default(),
            player_to_peer: HashMap::default(),
            peer_to_player: HashMap::default(),
            msg_receiver,
        };

        (connections, msg_sender)
    }

    pub async fn start_listening(&mut self) {
        info!("Started listening for connections");

        while let Some(request) = self.msg_receiver.recv().await {
            debug!("Received message request: {:?}", request);

            match request {
                WsCommand::SendMessage { msg, player_id } => {
                    self.send_message(msg, &player_id).await;
                }
                WsCommand::FetchAuthenticatedPlayer { peer_id, sender } => {
                    if let Err(err) =
                        sender.send(self.peer_to_player.get(&peer_id).map(String::clone))
                    {
                        error!("Sending authenticated player failed: {:?}", err);
                    }
                }
                WsCommand::AddConnection { sender, peer_id } => {
                    self.connections.insert(peer_id, sender);
                }
                WsCommand::RemoveConnection { peer_id } => {
                    self.connections.remove(&peer_id);
                    if let Some(player_id) = self.peer_to_player.get(&peer_id).map(String::clone) {
                        self.player_to_peer.remove(&player_id);

                        // about active players about left player
                        for peer_id in self.get_active_player_peer_ids() {
                            let mut player_msg = proto::message::Server_PlayerLostConn::new();
                            player_msg.set_player_id(player_id.clone());
                            let mut msg = proto::message::Server::new();
                            msg.set_playerLostConn(player_msg);
                            self.send_message(msg, &peer_id).await;
                        }
                    }
                    self.peer_to_player.remove(&peer_id);
                }
                WsCommand::RegisterActivePlayer { player, peer_id } => {
                    self.player_to_peer
                        .insert(String::from(player.id()), peer_id.clone());
                    self.peer_to_player
                        .insert(peer_id, String::from(player.id()));

                    // about active players about active, authorized player
                    for peer_id in self.get_active_player_peer_ids() {
                        let mut player_msg = proto::message::Server_PlayerEntered::new();
                        player_msg.set_player(player.clone().into());
                        let mut msg = proto::message::Server::new();
                        msg.set_playerEntered(player_msg);
                        self.send_message(msg, &peer_id).await;
                    }
                }
                WsCommand::IsActivePlayer { player_id, sender } => {
                    if let Err(err) = sender.send(self.player_to_peer.get(&player_id).is_some()) {
                        error!("Sending active player failed: {:?}", err);
                    }
                }
            }
        }
    }
    async fn send_message(&mut self, msg: proto::message::Server, peer_or_player_id: &str) {
        if let Err(err) = msg.check_initialized() {
            error!("Message not initialized correctly: {:?}", err);
            return;
        }

        if let Some(peer_id) = self
            .player_to_peer
            .get(peer_or_player_id)
            .map(String::clone)
            .or_else(|| {
                self.connections
                    .contains_key(peer_or_player_id)
                    .then(|| String::from(peer_or_player_id))
            })
        {
            match msg.write_to_bytes() {
                Ok(bytes) => {
                    if let Some(conn) = self.connections.get_mut(&peer_id) {
                        if let Err(err) = conn.send(WsMessage::binary(bytes)).await {
                            error!("Sending message to {} has failed: {:?}", &peer_id, &err);
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Writing message to binary format {} has failed: {:?}",
                        &peer_id, &err
                    );
                }
            }
        } else {
            debug!("Player {} has no active connection", peer_or_player_id);
        }
    }

    fn get_active_player_peer_ids(&self) -> Vec<String> {
        self.connections
            .iter()
            .filter(|(peer_id, _)| self.peer_to_player.contains_key(*peer_id))
            .map(|(x, _)| String::from(x))
            .collect::<Vec<_>>()
    }
}
