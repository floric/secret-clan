mod client;
mod connections;

use crate::model::{
    proto::{self},
    Player,
};
pub use client::WsClient;
pub use connections::Connections;
use futures::stream::SplitSink;
use tokio::sync::oneshot;
use warp::ws::{Message, WebSocket};

/// Message format to communicate the different command types for Websocket connections.
#[derive(Debug)]
pub enum WsCommand {
    SendMessage {
        player_id: String,
        msg: proto::message::Server,
    },
    RegisterActivePlayer {
        player: Player,
        peer_id: String,
    },
    FetchAuthenticatedPlayer {
        peer_id: String,
        sender: oneshot::Sender<Option<String>>,
    },
    IsActivePlayer {
        player_id: String,
        sender: oneshot::Sender<bool>,
    },
    AddConnection {
        peer_id: String,
        sender: SplitSink<WebSocket, Message>,
    },
    RemoveConnection {
        peer_id: String,
    },
}
