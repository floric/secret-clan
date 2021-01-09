use super::{Connections, WsCommand};
use crate::model::OutgoingMessage;
use futures::stream::SplitSink;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub struct WsClient {
    sender: mpsc::Sender<WsCommand>,
}

impl Default for WsClient {
    fn default() -> Self {
        let (mut connections, sender) = Connections::new();

        tokio::task::spawn(async move {
            connections.start_listening().await;
        });

        WsClient { sender }
    }
}

impl WsClient {
    pub async fn add_connection(
        &self,
        peer_id: &str,
        sender: SplitSink<WebSocket, Message>,
    ) -> Result<(), String> {
        self.sender
            .clone()
            .send(WsCommand::AddConnection {
                sender,
                peer_id: String::from(peer_id),
            })
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn remove_connection(&self, peer_id: &str) -> Result<(), String> {
        self.sender
            .clone()
            .send(WsCommand::RemoveConnection {
                peer_id: String::from(peer_id),
            })
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn send_message(
        &self,
        player_id: String,
        message: OutgoingMessage,
    ) -> Result<(), String> {
        self.sender
            .clone()
            .send(WsCommand::SendMessage {
                msg: message,
                player_id,
            })
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn register_active_player(
        &self,
        player_id: &str,
        peer_id: &str,
    ) -> Result<(), String> {
        self.sender
            .clone()
            .send(WsCommand::RegisterActivePlayer {
                peer_id: String::from(peer_id),
                player_id: String::from(player_id),
            })
            .await
            .map_err(|err| err.to_string())
    }
}
