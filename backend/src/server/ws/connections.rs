use super::WsCommand;
use futures::stream::SplitSink;
use futures::SinkExt;
use log::{error, info, warn};
use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub struct Connections {
    connections: HashMap<String, SplitSink<WebSocket, Message>>,
    player_connections: HashMap<String, String>,
    msg_receiver: mpsc::Receiver<WsCommand>,
}

impl Connections {
    pub fn new() -> (Self, mpsc::Sender<WsCommand>) {
        let (msg_sender, msg_receiver): (mpsc::Sender<WsCommand>, mpsc::Receiver<WsCommand>) =
            mpsc::channel(128);

        let connections = Connections {
            connections: HashMap::new(),
            player_connections: HashMap::new(),
            msg_receiver,
        };

        (connections, msg_sender)
    }

    pub async fn start_listening(&mut self) {
        info!("Started listening for connections");

        while let Some(request) = self.msg_receiver.recv().await {
            info!("Received message request");

            match request {
                WsCommand::SendMessage { msg, player_id } => {
                    if let Some(peer_id) = self.player_connections.get(&player_id) {
                        if self
                            .connections
                            .get_mut(peer_id)
                            .unwrap()
                            .send(Message::text(serde_json::to_string(&msg).unwrap()))
                            .await
                            .is_err()
                        {
                            error!("Sending message to {} has failed", &peer_id);
                        }
                    } else {
                        // TODO Maybe add retry here?
                        warn!("Player {} has no active connection", &player_id);
                    }
                }
                WsCommand::AddConnection { sender, peer_id } => {
                    self.connections.insert(peer_id, sender);
                }
                WsCommand::RemoveConnection { peer_id } => {
                    self.connections.remove(&peer_id);
                }
                WsCommand::RegisterActivePlayer { player_id, peer_id } => {
                    self.player_connections.insert(player_id, peer_id);
                }
            }
        }
    }
}
