use super::WsCommand;
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
                    if let Err(err) = msg.check_initialized() {
                        error!("Message not initialized correctly: {:?}", err);
                        continue;
                    }

                    if let Some(peer_id) = self.player_to_peer.get(&player_id).or_else(|| {
                        self.connections
                            .contains_key(&player_id)
                            .then(|| &player_id)
                    }) {
                        match msg.write_to_bytes() {
                            Ok(bytes) => {
                                if let Some(conn) = self.connections.get_mut(peer_id) {
                                    if let Err(err) = conn.send(WsMessage::binary(bytes)).await {
                                        error!(
                                            "Sending message to {} has failed: {:?}",
                                            &peer_id, &err
                                        );
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
                        debug!("Player {} has no active connection", &player_id);
                    }
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
                    if let Some(player_id) = self.peer_to_player.get(&peer_id) {
                        self.player_to_peer.remove(player_id);
                    }
                    self.peer_to_player.remove(&peer_id);
                }
                WsCommand::RegisterActivePlayer { player_id, peer_id } => {
                    self.player_to_peer
                        .insert(player_id.clone(), peer_id.clone());
                    self.peer_to_player.insert(peer_id, player_id);
                }
            }
        }
    }
}
