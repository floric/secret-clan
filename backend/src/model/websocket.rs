use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use warp::ws::{Message, WebSocket};

/// This tupel defines a directed request related to a Websocket-connection. The first argument defines the technical peer ID, not the users ID.
pub type WsCommand = (String, WsRequest);

/// All incoming message types which might be send by peers and should be handled on the server side.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum IncomingMessage {
    Auth { token: String },
}

/// All outgoing message types which might be send to the peers.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OutgoingMessage {
    Welcome {},
}

/// Message format to communicate the different command types for Websocket connections.
#[derive(Debug)]
pub enum WsRequest {
    SendMessage {
        msg: OutgoingMessage,
    },
    AddConnection {
        sender: SplitSink<WebSocket, Message>,
    },
}
