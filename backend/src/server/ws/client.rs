use super::{Connections, WsCommand};
use crate::model::proto::{self};
use futures::stream::SplitSink;
use log::error;
use tokio::sync::{mpsc, oneshot};
use warp::ws::{Message, WebSocket};

pub struct WsClient {
    pub sender: mpsc::Sender<WsCommand>,
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
        msg: proto::message::Server,
    ) -> Result<(), String> {
        self.sender
            .clone()
            .send(WsCommand::SendMessage { msg, player_id })
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

    pub async fn get_authenticated_player_for_peer(&self, peer_id: &str) -> Option<String> {
        let (sender, receiver) = oneshot::channel();
        if let Err(err) = self
            .sender
            .clone()
            .send(WsCommand::FetchAuthenticatedPlayer {
                peer_id: String::from(peer_id),
                sender,
            })
            .await
            .map_err(|err| err.to_string())
        {
            error!("Sending player authentication request failed: {:?}", err);
        }

        match receiver.await {
            Ok(res) => res,
            Err(err) => {
                error!("Receiving player ID failed: {:?}", err);
                None
            }
        }
    }
}
