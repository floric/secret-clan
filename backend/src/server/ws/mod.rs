mod client;
mod connections;

use crate::model::OutgoingMessage;
pub use client::WsClient;
pub use connections::Connections;
use futures::stream::SplitSink;
use warp::ws::{Message, WebSocket};

/// Message format to communicate the different command types for Websocket connections.
#[derive(Debug)]
pub enum WsCommand {
    SendMessage {
        player_id: String,
        msg: OutgoingMessage,
    },
    RegisterActivePlayer {
        player_id: String,
        peer_id: String,
    },
    AddConnection {
        peer_id: String,
        sender: SplitSink<WebSocket, Message>,
    },
    RemoveConnection {
        peer_id: String,
    },
}
